# Nashar-Gah
All Purpose Http Server


## M1 Macbook Tuning

```bash
# Increase TCP Buffer
sudo sysctl -w net.inet.tcp.sendspace=262144
sudo sysctl -w net.inet.tcp.recvspace=262144

# Increase number of sockets
ulimit -n 6553
```

## Performance Log
```bash
18th August 2024 (Returning Text)
projects/nashar_gah [main] $ wrk -t5 -c100 -d60s http://localhost:8080
Running 1m test @ http://localhost:8080
  5 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.44ms  330.79us   4.82ms   94.73%
    Req/Sec     6.06k     2.18k    7.54k    90.00%
  32805 requests in 1.00m, 3.79MB read
Requests/sec:    545.98
Transfer/sec:     64.51K
```