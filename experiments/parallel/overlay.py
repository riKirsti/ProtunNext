"""Build-only source transformation, never edits production src/vendor files.

Every replacement checks its anchor/count. The resulting archive and overlay
hashes are recorded in builds.json, so a detached-key experiment cannot be
mistaken for the unmodified source SHA.
"""
import hashlib,pathlib,shutil
HERE=pathlib.Path(__file__).resolve().parent
def apply(dest):
 def edit(path,old,new,count=1):
  file=dest/path;text=file.read_text();assert text.count(old)==count,(path,old,text.count(old));file.write_text(text.replace(old,new))
 noise=pathlib.Path('vendor/proton-boringtun/src/noise')
 for name in ['detached_session.rs','detached_tunn.rs']:
  shutil.copyfile(HERE/name,dest/noise/name)
 edit(noise/'session.rs','receiver: LessSafeKey,','receiver: std::sync::Arc<LessSafeKey>,')
 edit(noise/'session.rs','sender: LessSafeKey,','sender: std::sync::Arc<LessSafeKey>,')
 edit(noise/'session.rs','receiver: LessSafeKey::new(\n                UnboundKey::new(&CHACHA20_POLY1305, &receiving_key).unwrap(),\n            ),','receiver: std::sync::Arc::new(LessSafeKey::new(\n                UnboundKey::new(&CHACHA20_POLY1305, &receiving_key).unwrap(),\n            )),')
 edit(noise/'session.rs','sender: LessSafeKey::new(UnboundKey::new(&CHACHA20_POLY1305, &sending_key).unwrap()),','sender: std::sync::Arc::new(LessSafeKey::new(UnboundKey::new(&CHACHA20_POLY1305, &sending_key).unwrap())),')
 for path,include in [(noise/'session.rs','detached_session.rs'),(noise/'mod.rs','detached_tunn.rs')]:
  file=dest/path;file.write_text(file.read_text()+f'\ninclude!("{include}");\n')
 edit(pathlib.Path('src/session.rs'),'    tunnel: Tunn<ChaCha20Rng, Loose>,','    pub(crate) tunnel: Tunn<ChaCha20Rng, Loose>,')
 edit(pathlib.Path('src/session.rs'),'    fn output(result: TunnResult<\'_>)','    pub(crate) fn output(result: TunnResult<\'_>)')
 shutil.copyfile(HERE/'pool.rs',dest/'src/parallel_crypto.rs')
 file=dest/'src/lib.rs';file.write_text(file.read_text()+'\nmod parallel_crypto;\n')
 file=dest/'src/engine.rs';text=file.read_text()
 marker='    let mut pool = PacketPool::compact(132, (usize::from(config.mtu) + 32).max(2048), 65535)?;'
 assert text.count(marker)==1
 text=text.replace(marker,marker+'\n    let mut crypto_pool = crate::parallel_crypto::Pool::new()?;\n    let mut crypto_input = Vec::with_capacity(63);')
 # Retain all original policy, capture and output handling. Only gather the
 # already-approved TUN packets and execute the bounded crypto batch afterward.
 start=text.index('                    let (result, mut output) = encrypt_owned(&mut session, input)?;')
 end=text.index('\n                }\n                Err(e) if',start)
 old=text[start:end]
 text=text[:start]+'                    crypto_input.push(input);'+text[end:]
 marker='        #[cfg(feature = "control-plane")]\n        for _ in 0..BUDGET {\n            let Some(observer) = observer else {'
 assert text.count(marker)==1
 replacement='''        for (result, mut output) in crypto_pool.batch(&mut session, &mut crypto_input, true)?.drain(..) {
            let result = result?;
            counters.in_place_encryptions += 1;
            match result {
                Output::Network(n) => {
                    output.set_data_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, output, observer, &mut probe)?;
                    drain = true;
                }
                _ => pool.recycle(output)?,
            }
        }
'''
 text=text.replace(marker,replacement+marker)
 # Account for all deferred packets in the existing queue-pressure guard.
 anchor='|| to_net.len() + network.pending() >= 63'
 txstart=text.index('        for _ in 0..BUDGET {',text.index('    let mut network_error = None;'))
 # Only guard the TUN gather loop (not timer/control/network loops).
 gather=text.index('            let Some(device) = tun.as_mut() else {',txstart)
 guard=text.rfind(anchor,txstart,gather);assert guard>=0
 text=text[:guard]+text[guard:].replace(anchor,'|| to_net.len() + network.pending() + crypto_input.len() >= 63',1)
 # The first prototype parallelizes TX only. RX stays identical to main, including
 # replay handling. RX detached tickets are verified by regression tests before
 # enabling them in any worker. This avoids silently broadening the measured path.
 file.write_text(text)
 return {p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in HERE.glob('*') if p.is_file()}
