# alta

alta is Access Log Timeseries Analyzser for Labeled Tab-separated Values(LTSV).

Inspired by [alp](https://github.com/tkuchiki/alp).

# Installation

## Requirement

alta requires `gnuplot` command.

See http://www.gnuplot.info/ .

## Build from source

```
$ git clone git@github.com:genya0407/alta.git
$ cd alta
$ cargo build --release
$ ./target/release/alta
```

# Usage

Read from stdin or an input file(`-f`).

```
$ ./alta --help
Usage: ./alta [options]

Options:
    -h, --help          print this help menu
        --sum           show the sum of the response times
        --cnt           show the count of the requests
        --avg           show the average response time
        --max           show the average response time
        --min           show the average response time
        --uri PATTERN   set target uri pattern
        --method METHOD set target http method
        --status STATUS set target http status
        --width WIDTH   set graph width (default: 1.0)
    -f, --input-file FILE
                        set nginx log file
```

# Log format

## Nginx

```
log_format ltsv "time:$time_local"
                "\thost:$remote_addr"
                "\tforwardedfor:$http_x_forwarded_for"
                "\treq:$request"
                "\tstatus:$status"
                "\tsize:$body_bytes_sent"
                "\treferer:$http_referer"
                "\tua:$http_user_agent"
                "\treqtime:$request_time"
                "\tcache:$upstream_http_x_cache"
                "\truntime:$upstream_http_x_runtime"
                "\tvhost:$host";

access_log /var/log/nginx/access.log ltsv;
```

# Example

## Aggregation method

### Request count

Plot request count per second in timeseries.

`--uri=/icons` matches `/icons` or `/icons/hoge.png` etc.

```
$ ./alta -f access.log --cnt --uri=/icons


  1200 +----------------------------------------------------------------------------------------+
       |              +              +              +             +              +              |
       |                                                               "<cat" using 1:2 ******* |
       |                                                                                        |
  1000 |-+    *                                                                               +-|
       |      *                                                                                 |
       |      *                                                                                 |
       |      *                                                                                 |
       |      **                                                                                |
   800 |-+    **                                                                              +-|
       |      **                                                                                |
       |      **                                                                                |
       |      **                                                                                |
   600 |-+    **                                                                              +-|
       |      **                                                                                |
       |     * *                                                                                |
       |     *  *                                                                               |
   400 |-+   *  *            **                                                               +-|
       |     *  *            * *                                                                |
       |     *  *         ***  *            **          ***                                     |
       |     *   *       *      *          *  *         *  *                                    |
       |     *    *     *       *          *   *       *   *                                    |
   200 |-+   *    *    *        **        *    *       *    *                                 +-|
       |     *     * **           *** *** *     * *** *     ***       *****         *****  **** |
       |     *     **                *   *       *   *         ***** *     *****  **     **    *|
       |   ***        +              +   *          +             + *           **              |
     0 +----------------------------------------------------------------------------------------+
     42:20          42:30          42:40          42:50         43:00          43:10          43:20
```

### Max response time

Plot max response time in every 1 second.

```
$ ./alta -f access.log --max --uri=/icons


  0.8 +-----------------------------------------------------------------------------------------+
      |              +              +              +              +              +              |
      |       *                                                        "<cat" using 1:2 ******* |
  0.7 |-+     *                                                                               +-|
      |       *                                                                                 |
      |       *                                                                                 |
  0.6 |-+     *                                                                               +-|
      |      **                                                                                 |
      |      **                                                                                 |
      |      **                                                                                 |
  0.5 |-+    **                                                                               +-|
      |      **                                                                                 |
      |      **                                                                                 |
  0.4 |-+    **                                                                               +-|
      |      * *                                                                                |
      |     *  *                                           *                                    |
  0.3 |-+   *  *                                           *                                  +-|
      |     *  *                                          **                                    |
      |     *  *                                          **                                    |
  0.2 |-+   *  *                                         * *                                  +-|
      |    *   *                                         *  *                                   |
      |    *   *                                         *  *                                   |
      |   *    *                                         *  *                                   |
  0.1 |-+ *    *                                *       *   *                                 +-|
      |  *     *              ***    **      ** ** **   *   *                                   |
      |  *     *     +   ** **   ** *  *  ***  *  *+ ****   ******** ****** *****+  *****  **** |
    0 +-----------------------------------------------------------------------------------------+
    42:20          42:30          42:40          42:50          43:00          43:10          43:20
```

### Others

alta also currently supports `avg`, `sum` and `min`.

## Filter data

alta can filter data with `uri`, `method`, `status`.

```
$ ./alta -f access.log --cnt --uri=/icons --method=GET --status=200


  160 +-----------------------------------------------------------------------------------------+
      |              +              +              +              +              +              |
      |                                                  *             "<cat" using 1:2 ******* |
  140 |-+                                                **                                   +-|
      |                                                 *  *                                    |
      |                                                 *  *                                    |
  120 |-+                                               *  *                                  +-|
      |                                                 *  *                                    |
      |                                                *   *                                    |
      |                                                *    *                                   |
  100 |-+                                              *    *                                 +-|
      |       *                                        *    *                                   |
      |       *                                        *    *                                   |
   80 |-+     *                              **        *    *                                 +-|
      |      **                              * *      *     *                                   |
      |      **                             *  *      *      *                                  |
   60 |-+    **                             *  *      *       *         **          *         +-|
      |      **                            *    *     *        *      **  *         **          |
      |     * *                            *    *     *        *     *    *  *      **          |
   40 |-+   *  *                           *    *  *  *         * *  *     ** *     * *       +-|
      |     *  *                     **   *     *** **          * ** *     ** *    *   **   *   |
      |    *   *               ** *** *   *          *           * **      *   *   *     * * *  |
      |   *    *              *  *    *   *                         *           * **      **  * |
   20 |-+ *    *              *        * *                          *            *        *   +*|
      |  *     *              *        * *                                                     *|
      |**      *     +   **  *      +  **          +              +              +              |
    0 +-----------------------------------------------------------------------------------------+
    42:20          42:30          42:40          42:50          43:00          43:10          43:20
```