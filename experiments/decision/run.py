import subprocess,sys,json,time
binary=sys.argv[1]; started=time.monotonic(); errors=[]
cases=[('single',0,0,16),('batch',0,0,16),('batch',1,0,16),('batch',0,1,16),('batch',1,1,16),('uring16',0,0,16),('uring128',0,0,128),('uring128',1,0,128),('uring128',0,1,128),('uring128',1,1,128)]
for r in range(4):
 for size in (64,1280):
  for mode,tx,rx,buffers in (cases if r%2==0 else cases[::-1]):
   if time.monotonic()-started>240: print('DECISION_BUDGET_EXHAUSTED',flush=True);sys.exit(3)
   args=[binary,'--udp',mode,str(size),str(r),str(tx),str(rx),str(buffers)]
   try:
    p=subprocess.run(args,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=15)
    print(p.stdout,flush=True)
    if p.returncode: errors.append({'args':args[1:],'exit':p.returncode})
   except subprocess.TimeoutExpired: errors.append({'args':args[1:],'error':'timeout'})
print('DECISION_ERRORS',json.dumps(errors),flush=True)
sys.exit(bool(errors))
