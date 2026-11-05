"""Bounded loopback-only kTLS worker overlay; shipping source stays untouched."""
import hashlib,pathlib,shutil
HERE=pathlib.Path(__file__).resolve().parent
def apply(dest):
 def edit(path,old,new,count=1):
  file=dest/path;s=file.read_text();assert s.count(old)==count,(path,old,s.count(old));file.write_text(s.replace(old,new))
 shutil.copyfile(HERE/'adapter.rs',dest/'src/ktls_worker.rs')
 (dest/'src/ktls_worker').mkdir()
 shutil.copyfile(HERE.parent/'ktls/src/ktls.rs',dest/'src/ktls_worker/core.rs')
 file=dest/'src/lib.rs';file.write_text(file.read_text()+'\n#[cfg(target_os="linux")] mod ktls_worker;\n')
 edit(pathlib.Path('src/linux.rs'),'crate::tls::framed(socket,','crate::ktls_worker::framed(socket,',2)
 edit(pathlib.Path('src/tls.rs'),'    config.alpn_protocols.clear();','    config.enable_secret_extraction = true;\n    config.alpn_protocols.clear();')
 edit(pathlib.Path('src/stealth.rs'),'        let config = rustls::ClientConfig::builder_with_provider(provider.clone())','        let mut config = rustls::ClientConfig::builder_with_provider(provider.clone())')
 edit(pathlib.Path('src/stealth.rs'),'        Ok(Arc::new(config))','        config.enable_secret_extraction = true;\n        Ok(Arc::new(config))')
 files=[p for p in HERE.glob('*')if p.is_file()]+[HERE.parent/'ktls/src/ktls.rs']
 return {str(p.relative_to(HERE.parent)):hashlib.sha256(p.read_bytes()).hexdigest()for p in files}
