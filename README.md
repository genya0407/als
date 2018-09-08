# alta

alta is Access Log Timeseries Analyzser for Labeled Tab-separated Values(LTSV).

Inspired by [alp](https://github.com/tkuchiki/alp).

# Requirement

alta requires `gnuplot` command.

See http://www.gnuplot.info/

# Installation

Download from https://github.com/genya0407/alta/releases/latest

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

Shows request count in every 1 second in timeseries.

```
$ ./alta -f access.log --cnt


  1600 +----------------------------------------------------------------------------------------+
       |              +              +              +             +              +              |
       |                                                               "<cat" using 1:2 ******* |
  1400 |-+    *                                                                               +-|
       |      *                                                                                 |
       |      *                                                                                 |
  1200 |-+    **                                                                              +-|
       |      **             **                                                                 |
       |      **             * *                                                                |
       |      **             * *            *                                                   |
  1000 |-+    * *           *  *            *                                                 +-|
       |      * *         ***  *            **          *                                       |
       |     *  *         *    *            **          *                                       |
   800 |-+   *  *        *     *           *  *         *                                     +-|
       |     *   *      *       *          *  *        * *                                      |
       |     *   *     *        *          *  *        * ***                                    |
   600 |-+   *   *     *        *          *  *        * * *                                  +-|
       |     *   *     *        *          *   *       *   *                                    |
       |     *    *    *        *         *    *      *     *                                   |
   400 |-+   *    *   *         *         *    *      *     *                                 +-|
       |     *    *   *          *        *     *     *     *          *                        |
       |     *    *   *          *    **  *     * **  *     ***       * *            **         |
       |    *      * *            * **  **       ** **         *     *  ********    *  **  **** |
   200 |-+  *      **              *     *       *   *         ***** *          * **     **   +*|
       |    *                                                       *            *              |
       |** *          +              +              +             +              +              |
     0 +----------------------------------------------------------------------------------------+
     42:20          42:30          42:40          42:50         43:00          43:10          43:20
```

### Max response time

Shows max response time in every 1 second.

```
$ ./alta -f access.log --max


  10 +------------------------------------------------------------------------------------------+
     |              +              +               +              +              +              |
     |                                                                 "<cat" using 1:2 ******* |
   9 |-+                    **********   *******   *************** *********************** *****|
     |                      *        *   *      * *               *                       *     |
   8 |-+                    *        *   *      * *                                           +-|
     |                      *        *   *       *                                              |
     |                      *        *   *                                                      |
   7 |-+                    *        *  *                                                     +-|
     |                     *          * *                                                       |
   6 |-+                   *          * *                                                     +-|
     |                     *          * *                                                       |
     |                     *          * *                                                       |
   5 |-+                   *          **                                                      +-|
     |                     *          *                                                         |
     |                     *                                                                    |
   4 |-+                   *                                                                  +-|
     |                     *                                                                    |
   3 |-+            *      *                                                                  +-|
     |            ** *     *                                                                    |
     |           *   *     *                                                                    |
   2 |-+      ***     **  *                                                                   +-|
     |        *        *  *                                                                     |
   1 |-+  ****          * *                                                                   +-|
     |   *   *          * *                                                                     |
     |   *          +    **        +               +              +              +              |
   0 +------------------------------------------------------------------------------------------+
   42:20          42:30          42:40           42:50          43:00          43:10          43:20
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