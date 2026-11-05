"""Exit owned QEMU after the guest success/failure marker, even without ACPI."""
import subprocess,sys,time,selectors,re
kernel,initrd,log=sys.argv[1:]
args=['sudo','-n','qemu-system-x86_64','-accel','kvm','-cpu','host','-m','2048','-smp','4','-nographic','-no-reboot','-object','rng-random,filename=/dev/urandom,id=rng0','-device','virtio-rng-pci,rng=rng0','-nic','none','-kernel',kernel,'-initrd',initrd,'-append','console=ttyS0 rdinit=/init panic=-1']
p=subprocess.Popen(args,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,stdin=subprocess.DEVNULL,bufsize=0)
selector=selectors.DefaultSelector();selector.register(p.stdout,selectors.EVENT_READ)
data=b'';mark=None;start=time.monotonic()
try:
 with open(log,'wb') as f:
  while time.monotonic()-start<540:
   if selector.select(1):
    chunk=p.stdout.read(65536)
    if not chunk:break
    f.write(chunk);f.flush();sys.stdout.buffer.write(chunk);sys.stdout.buffer.flush();data+=chunk
    mark=re.search(rb'(?:^|\n)DECISION_FINISHED ([0-9]+)\r?\n',data)
    if mark:break
   elif p.poll() is not None:break
finally:
 if p.poll() is None:
  p.terminate()
  try:p.wait(timeout=5)
  except subprocess.TimeoutExpired:p.kill();p.wait()
if mark is None:raise SystemExit('Guest did not reach final marker')
raise SystemExit(int(mark.group(1)))
