# Performance Log For Nashargah

## M1 Macbook Tuning

```bash
# Increase TCP Buffer
sudo sysctl -w net.inet.tcp.sendspace=262144
sudo sysctl -w net.inet.tcp.recvspace=262144

# Increase number of sockets
ulimit -n 6553
```

### 18th August 2024 

Returning Text

```bash
projects/nashar_gah [main] $ wrk -t5 -c100 -d60s http://localhost:8080
```

| Metric             | Value        |
|--------------------|--------------|
| **Requests/sec**   | 545.98       |
| **Transfer/sec**   | 64.51K       |
| **Total Requests** | 32,805       |
| **Data Transferred** | 3.79MB      |


### 19th August 2024 

Returning a simple html page

```bash
projects/nashar_gah [main] $ wrk -t5 -c100 -d60s http://localhost:8080
```

| Metric             | Value        |
|--------------------|--------------|
| **Requests/sec**   | 122,106.62   |
| **Transfer/sec**   | 431.80M      |
| **Total Requests** | 7,334,360    |
| **Data Transferred** | 25.33GB     |


###  20th August

Load Testing, sending a html file and Benchmarking

```bash
projects/nashar_gah [main] $ wrk -t8 -c500 -d15m http://localhost:8080
```

| **Metric**                  | **NasharGah (Rust)** | **Go (std)** | **Python (Flask - Gunicorn - Gevent)** |
|-----------------------------|----------------------|--------------|----------------------------------------|
| **RPS**                     | 112,358.62           | 64,853.99    | 6,604.18                               |
| **Latency (Avg)**           | 5.26 ms              | 9.99 ms      | 116.03 ms                              |
| **Latency (Stdev)**         | 6.73 ms              | 11.78 ms     | 133.06 ms                              |
| **Latency (Max)**           | 208.31 ms            | 242.86 ms    | 2.00 s                                 |
| **Throughput**              | 397.33 MB/sec        | 234.84 MB/sec| 24.90 MB/sec                           |
| **Total Requests**          | 101,128,099          | 58,373,504   | 5,943,958                              |
| **Total Data Transferred**  | 349.23 GB            | 206.42 GB    | 21.89 GB                               |
| **Socket Errors (Read)**    | 656                  | 664          | 1,090                                  |
| **Socket Errors (Write)**   | 0                    | 0            | 153                                    |
| **Socket Errors (Timeout)** | 0                    | 0            | 94                                     |

###  22nd August

Load Testing, sending a html file and Benchmarking

```bash
projects/nashar_gah [main] $ wrk -t8 -c500 -d15m http://localhost:8080
```

| Metric             | Value          |
|--------------------|----------------|
| **Test Duration**  | 15 minutes      |
| **Threads**        | 8              |
| **Connections**    | 500            |
| **Avg Latency**    | 5.21 ms        |
| **Avg Req/Sec**    | 14.80k         |
| **Total Requests** | 105.4M         |
| **Total Data**     | 364.1 GB       |
| **Socket Errors**  | Read: 648      |
| **Req/Sec**        | 117,147.01     |
| **Transfer/Sec**   | 414.26 MB      |
