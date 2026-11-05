"""Boot cached actual 6.18 kernel; pin each virtual CPU before guest execution."""
import json,os,pathlib,socket,subprocess,time
R=pathlib.Path(__file__).resolve().parents[2];O=R/'target/audit-lts';O.mkdir(parents=True,exist_ok=True)
cpus=sorted(os.sched_getaffinity(0));assert len(cpus)>=4
image=R/'target/ktls-worker-kernels/6.18.60/bzImage';assert image.is_file(),'verified TUN/TLS kernel cache required'
guest=O/'root';(guest/'bin').mkdir(parents=True,exist_ok=True)
for folder in ['proc','sys','dev','tmp']:(guest/folder).mkdir(exist_ok=True)
import shutil
shutil.copyfile('/usr/bin/busybox',guest/'bin/busybox');os.chmod(guest/'bin/busybox',0o755)
for app in ['sh','mount','ip','poweroff','uname','cat','unshare']:
 p=guest/'bin'/app
 if not p.exists():p.symlink_to('busybox')
for name in ['erase','ktls']:shutil.copyfile(R/'target/audit-v2/bin'/name,guest/'bin'/name);os.chmod(guest/'bin'/name,0o755)
lines=['#!/bin/sh','export PATH=/bin','mount -t proc proc /proc','mount -t sysfs sysfs /sys','mount -t devtmpfs devtmpfs /dev','export NEXT_PEER_CPUS=0 NEXT_LOAD_CPUS=1 NEXT_CLIENT_CPUS=2,3 NEXT_WARM_SECONDS=1','echo AUDIT_KERNEL $(uname -r)','result=0']
# A/A and matched implementation blocks repeated within one host allocation.
for repeat in range(3):
 for shape,flows,size,window in [('burst',1,1200,32),('sliding',1,1200,32),('sliding',8,1200,32),('burst',1,64,1)]:
  for role,order in [('aa',['erase']*4),('ktls',['erase','ktls','ktls','erase'])]:
   for slot,name in enumerate(order):
    label=json.dumps(dict(repeat=repeat,shape=shape,flows=flows,size=size,window=window,role=role,slot=slot,build=name),separators=(',',':'))
    lines += ["echo 'AUDIT_CASE "+label+"'",f'NEXT_LOAD_SHAPE={shape} unshare -n /bin/{name} client /bin/erase stealth {size} {window} 5 {flows} || result=1']
lines += ['cat /proc/net/tls_stat','echo AUDIT_EXIT $result','poweroff -f']
(guest/'init').write_text('\n'.join(lines)+'\n');os.chmod(guest/'init',0o755)
subprocess.run('find . -print0 | cpio --null -o --format=newc | gzip -1 >../initramfs.gz',shell=True,cwd=guest,check=True)
qmp=O/'qmp.sock';assert not qmp.exists()
cmd=['qemu-system-x86_64','-accel','kvm','-cpu','host','-m','1536','-smp','4','-nographic','-no-reboot','-S','-qmp',f'unix:{qmp},server=on,wait=off','-object','rng-random,filename=/dev/urandom,id=rng0','-device','virtio-rng-pci,rng=rng0','-nic','none','-kernel',str(image),'-initrd',str(O/'initramfs.gz'),'-append','console=ttyS0 rdinit=/init panic=-1 quiet']
(O/'host-before.txt').write_text(pathlib.Path('/proc/stat').read_text())
with (O/'guest.log').open('w') as log:
 p=subprocess.Popen(cmd,stdout=log,stderr=subprocess.STDOUT)
 try:
  deadline=time.monotonic()+10
  while not qmp.exists():assert time.monotonic()<deadline;time.sleep(.05)
  s=socket.socket(socket.AF_UNIX);s.connect(str(qmp));f=s.makefile('rwb',buffering=0);json.loads(f.readline())
  def call(name):
   f.write((json.dumps({'execute':name})+'\n').encode())
   while True:
    reply=json.loads(f.readline())
    if 'return'in reply:return reply['return']
    assert 'error' not in reply,reply
  call('qmp_capabilities');vcpus=call('query-cpus-fast');pins=[]
  for v in vcpus:
   physical=cpus[v['cpu-index']];os.sched_setaffinity(v['thread-id'],{physical});pins.append(dict(vcpu=v['cpu-index'],thread=v['thread-id'],host_cpu=physical))
  (O/'pins.json').write_text(json.dumps(pins,indent=2));call('cont');s.close();assert p.wait(timeout=1000)==0
 finally:
  if p.poll() is None:p.kill();p.wait()
(O/'host-after.txt').write_text(pathlib.Path('/proc/stat').read_text())
shutil.copyfile(R/'target/ktls-worker-kernels/6.18.60/config',O/'kernel.config')
assert 'AUDIT_EXIT 0' in (O/'guest.log').read_text()
