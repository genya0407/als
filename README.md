# alta

alta is Access Log Timeseries Analyzser for Labeled Tab-separated Values(LTSV).

Inspired by [alp](https://github.com/tkuchiki/alp).

# Installation

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

## Request count

Show as tsv:

```
$ cat access.log | ./alta --cnt
1535787740  104
1535787741  31
1535787742  5
1535787743  2
1535787744  358
1535787745  1443
1535787746  875
1535787747  600
1535787748  182
1535787749  161
1535787750  330
1535787751  656
1535787752  809
1535787753  940
...
```

Plot as a graph using `./bin/plot`:

```
$ > cat access.log | ./alta --cnt | ./bin/plot


  1600 +------------------------------------------------------------------------------------------------------------------------------------------+
       |                      +                      +                       +                      +                      +                      |
       |                                                                                                                 "<cat" using 1:2 ******* |
  1400 |-+         *                                                                                                                            +-|
       |           *                                                                                                                              |
       |           *                                                                                                                              |
  1200 |-+        * *                                                                                                                           +-|
       |          * *                     **                                                                                                      |
       |          * *                    *  *                                                                                                     |
       |          * *                    *  *                    *                                                                                |
  1000 |-+        *  *                  *   *                    *                                                                              +-|
       |          *  *               ***     *                  * *                *                                                              |
       |         *   *              *        *                  * *                **                                                             |
   800 |-+       *    *            *         *                  *  *               **                                                           +-|
       |         *    *          **          *                  *  *              *  *                                                            |
       |         *     *        *            *                 *    *             *  * **                                                         |
   600 |-+       *     *        *            *                 *    *             *   * *                                                       +-|
       |         *     *       *              *                *     *            *      *                                                        |
       |        *       *      *              *               *      *           *       *                                                        |
   400 |-+      *       *      *              *               *       *          *       *                                                      +-|
       |        *        *    *                *             *         *         *        *                *                                      |
       |        *        *    *                 *      ***   *         *  ***   *         ****           ** *                    ***              |
       |       *          *  *                   ** ***   ***           **   ** *             *         *    ************     ***   **    ******  |
   200 |-+     *          ***                      *        *           *      *               ******* *                 ** **        ****      *-|
       |*      *                                                                                      *                    *                     *|
       | ***  *               +                      +                       +                      +                      +                      |
     0 +------------------------------------------------------------------------------------------------------------------------------------------+
     42:20                  42:30                  42:40                   42:50                  43:00                  43:10                  43:20
```

## Response time

Show average response time:

```
$ cat access.log | ./alta --avg | ./bin/plot


  3.5 +-------------------------------------------------------------------------------------------------------------------------------------------+
      |                      +                       +                      +                      +                       +                      |
      |                                                                                                                  "<cat" using 1:2 ******* |
      |                                                                                                                                           |
    3 |-+                                                                                                                                       +-|
      |                                                                                                                                           |
      |                                                                                                                                           |
  2.5 |-+                                                                                                                                       +*|
      |                                                                                                                                          *|
      |                                                                                                                                          *|
      |                                                                                                                                          *|
    2 |-+                                                                                                                                       +*|
      |                                                                                                                                          *|
      |                                                                                                                                          *|
      |                                                                                                                                          *|
  1.5 |-+                                                                                                                                       +*|
      |                                                                                                                                          *|
      |                                                                                                                                          *|
      |                                                                                                                                          *|
    1 |-+                                                                                                                                       *-|
      |                                                                                                                                         * |
      |                                                                                                                                         * |
  0.5 |-+    *                                                                                  **    *       ******               ******       *-|
      |      **                                ******                 **** **             ******  * ** *    **       ***  *********      ****   * |
      |     * *                              **       ***           **    *  *           *         *    ****        *   **                   **** |
      |     *  *        ********          ***        *   ***********        + ***** *****          +                       +                      |
    0 +-------------------------------------------------------------------------------------------------------------------------------------------+
    42:20                  42:30                   42:40                  42:50                  43:00                   43:10                  43:20
```

## Filtering

Filter by uri, status code and method:

```
$ cat access.log | ./alta --cnt --uri=/icons --status=304 --method=GET | ./bin/plot


  1000 +------------------------------------------------------------------------------------------------------------------------------------------+
       |           *          +           +          +           +           +          +           +          +           +          +           |
       |           *                                                                                                     "<cat" using 1:2 ******* |
   900 |-+         *                                                                                                                            +-|
       |           *                                                                                                                              |
   800 |-+         **                                                                                                                           +-|
       |          * *                                                                                                                             |
       |          * *                                                                                                                             |
   700 |-+        * *                                                                                                                           +-|
       |          * *                                                                                                                             |
   600 |-+        * *                                                                                                                           +-|
       |          * *                                                                                                                             |
       |          * *                                                                                                                             |
   500 |-+        *  *                                                                                                                          +-|
       |         *   *                                                                                                                            |
       |         *   *                    *                                                                                                       |
   400 |-+       *   *                   * *                                                                                                    +-|
       |         *   **                 *   *                                                                                                     |
   300 |-+       *     *           *****    *                                                                                                   +-|
       |         *      *         *          *                                                                                                    |
       |         *      *         *          *                 *****                                                                              |
   200 |-+       *       *       *            *               *     *              ***                                                          +-|
       |        *        *    ***             ***             *      *            *   ***                                                         |
   100 |-+      *         ** *                   **     ***  *        ** *****    *      *                                    ******        ****+-|
       |        *           *                      ** **   * *          *     * **        ***** ****** ******************   **      ***** **    * |
       |        *  +          +           +          *      *    +           + *        +      *    + *        +         ***          +  *        |
     0 +------------------------------------------------------------------------------------------------------------------------------------------+
     42:20       42:25      42:30       42:35      42:40       42:45       42:50      42:55       43:00      43:05       43:10      43:15       43:20
```