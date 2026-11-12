"""Independent client PGO/CPU builds with unmodified baseline helper binaries."""
import json
import os
import pathlib
import re
import shlex
import shutil
import subprocess
import sys
import tempfile

from common import cpu_identity, sha

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT/'target/erasure-audit'
sys.path.insert(0, str(ROOT/'experiments/audit'))
from prepare import prepare


def main():
    target = sys.argv[1]
    mode = os.environ['NEXT_BUILD_MODE']
    assert mode in {'pgo', 'cpu'}
    assert os.environ.get('NEXT_RECEIVE_PROTOTYPE', '0') != '1' and os.environ.get('NEXT_TLS_RECORD_PROTOTYPE', '0') != '1', 'compare build changes independently'
    compiler = subprocess.check_output(['rustc', '-vV'], text=True)
    host = next(line.split(': ', 1)[1] for line in compiler.splitlines() if line.startswith('host:'))
    assert target.split('-')[0] == host.split('-')[0], 'training must run on the target architecture'
    if mode == 'cpu':
        assert target == host, 'host-native instructions are not a virtual-guest/cross-device contract'
    flags = shlex.split(os.environ.get('RUSTFLAGS', ''))
    assert not any('profile-' in f or 'target-cpu' in f or 'target-feature' in f for f in flags), 'undeclared inherited tuning flags'
    state = prepare('HEAD', os.environ['BASELINE'])
    assert state['main']['runtime_sources_sha256'] == state['erase']['runtime_sources_sha256'], 'build experiment must retain identical runtime source'
    (OUT/'bin').mkdir(exist_ok=True)
    (OUT/'toolchain.txt').write_text(compiler+f'\ntarget={target}\nprofile=release thin-LTO codegen-units=1 panic=abort\nbase RUSTFLAGS={flags!r}\nmode={mode}\n')
    base_env = dict(os.environ)
    base_env.pop('CARGO_ENCODED_RUSTFLAGS', None)

    def build(name, selected, log, test=False, example=False):
        env = dict(base_env, CARGO_TARGET_DIR=str(OUT/f'cargo-{name}'),
                   NEXT_BUILD_ID=state[name]['effective_sources_sha256'], RUSTFLAGS=shlex.join(selected))
        args = ['cargo', 'test' if test else 'build', '--locked', '--release', '--target', target,
                '--no-default-features', '--features', 'unix', '--manifest-path', str(pathlib.Path(state[name]['path'])/'Cargo.toml')]
        args += ['--lib'] if test else (['--example', 'erasure_codegen'] if example else ['--bin', 'erasure-probe'])
        with (OUT/log).open('w') as stream:
            subprocess.run(args, env=env, cwd=ROOT, stdout=stream, stderr=subprocess.STDOUT, check=True)
        return OUT/f'cargo-{name}'/target/'release'/('examples/erasure_codegen' if example else 'erasure-probe')

    build('erase', flags, 'probe-regressions.log', test=True)
    baseline = build('main', flags, 'build-main.log')
    shutil.copy2(baseline, OUT/'bin/main')
    state['main']['build_configuration'] = dict(mode='release', target=target, compiler=compiler,
                                               rustflags=flags, binary_sha256=sha(OUT/'bin/main'))
    extra = dict()
    if mode == 'cpu':
        selected = flags+['-Ctarget-cpu=native']
        cfg = subprocess.check_output(['rustc', '--print', 'cfg', '--target', target, '-Ctarget-cpu=native'], text=True)
        extra.update(target_cpu='native', cpu_identity=cpu_identity(), compiler_target_cfg=cfg.splitlines())
    else:
        folder = pathlib.Path(tempfile.mkdtemp(prefix='pgo-', dir=OUT))
        (folder/'raw').mkdir()
        generated = flags+['-Cprofile-generate='+str(folder/'raw')]
        instrumented = build('erase', generated, 'pgo-generate.log')
        shutil.copy2(instrumented, folder/'instrumented-client')
        trainer = ['sudo', 'env', 'PATH='+os.environ['PATH'], sys.executable,
                   str(ROOT/'experiments/build/train.py'), str(folder/'instrumented-client'),
                   str(OUT/'bin/main'), str(folder), state['erase']['effective_sources_sha256'],
                   state['main']['effective_sources_sha256']]
        subprocess.run(trainer, cwd=ROOT, check=True)
        profiles = sorted((folder/'raw').glob('*.profraw'))
        assert len(profiles) >= 6 and all(p.stat().st_size > 0 for p in profiles), 'missing fresh training counters'
        sysroot = pathlib.Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip())
        tool = sysroot/'lib/rustlib'/host/'bin/llvm-profdata'
        version = subprocess.check_output([str(tool), '--version'], text=True)
        expected = re.search(r'LLVM version: (\d+\.\d+\.\d+)', compiler).group(1)
        assert expected in version, 'LLVM tools do not match compiler'
        merged = folder/'client.profdata'
        subprocess.run([str(tool), 'merge', '-o', str(merged), *map(str, profiles)], check=True)
        summary = subprocess.check_output([str(tool), 'show', '--detailed-summary', str(merged)], text=True)
        assert int(re.search(r'Total functions:\s*(\d+)', summary).group(1)) > 0
        assert int(re.search(r'Maximum function count:\s*(\d+)', summary).group(1)) > 0
        (folder/'profile-summary.txt').write_text(summary)
        selected = flags+['-Cprofile-use='+str(merged), '-Cllvm-args=-pgo-warn-missing-function']
        extra.update(profile=dict(path=str(merged), sha256=sha(merged), llvm_tool_sha256=sha(tool),
                                  llvm_version=version, instrumented_binary_sha256=sha(folder/'instrumented-client'),
                                  generation_flags=generated,
                                  raw_files=[dict(name=p.name, bytes=p.stat().st_size, sha256=sha(p)) for p in profiles],
                                  training=json.loads((folder/'training.json').read_text())))
    final = build('erase', selected, 'build-erase.log')
    shutil.copy2(final, OUT/'bin/erase')
    build('erase', selected, 'optimized-regressions.log', test=True)
    erasure = build('erase', selected, 'optimized-erasure-build.log', example=True)
    subprocess.run([str(erasure), 'K'], check=True)
    assembly = subprocess.check_output(['objdump', '-Cd', '--no-show-raw-insn', str(erasure)], text=True)
    relocations = subprocess.run(['objdump', '-R', str(erasure)], capture_output=True, text=True)
    assembly += relocations.stdout
    assembly_path = OUT/'optimized-erasure-disassembly.txt'
    assembly_path.write_text(assembly)
    with (OUT/'optimized-erasure-check.log').open('w') as stream:
        subprocess.run([sys.executable, str(ROOT/'experiments/audit/check_codegen.py'), str(assembly_path)], stdout=stream, stderr=subprocess.STDOUT, check=True)
    extra.update(erasure_checked=True, erasure_binary_sha256=sha(erasure))
    if mode == 'pgo':
        section_tool = tool.with_name('llvm-objdump')
        generated_sections = subprocess.check_output([str(section_tool), '-h', str(folder/'instrumented-client')], text=True)
        final_sections = subprocess.check_output([str(section_tool), '-h', str(OUT/'bin/erase')], text=True)
        assert '__llvm_prf_cnts' in generated_sections, 'training binary lacks instrumentation'
        assert '__llvm_prf_cnts' not in final_sections, 'final binary retains instrumentation'
        (folder/'generated-sections.txt').write_text(generated_sections)
        (folder/'final-sections.txt').write_text(final_sections)
        extra['profile']['instrumentation_absent'] = True
    state['erase']['build_configuration'] = dict(mode=mode, target=target, compiler=compiler,
                                                rustflags=selected, binary_sha256=sha(OUT/'bin/erase'), **extra)
    assert state['erase']['build_configuration']['binary_sha256'] != state['main']['build_configuration']['binary_sha256'], 'optimized artifact is unchanged'
    (OUT/'builds.json').write_text(json.dumps(state, indent=2))
    print(f'Built independent {mode} client and fixed release helpers; retained compiler/profile/binary provenance.')


if __name__ == '__main__':
    main()
