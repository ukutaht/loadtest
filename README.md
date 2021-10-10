# Plausible Analytics load test


This project aims to create a load testing harness for Plausible Analytics. The load test enables us to figure out the limits of how
much traffic one server can absorb and hopefully pinpoints us to bottlenecks that we can optimize.

The test setup uses a Rust based load testing tool called [Goose](https://github.com/tag1consulting/goose).

### Mimicing real-world usage

Care must be taken to make sure we are actually exercising the parts of code that get red hot under load. With this regard, here
is the thinking in how the tests are setup.

| Property   | Load test behaviour                                  | Potential bottleneck |
|------------|------------------------------------------------------|----------------------|
| IP address | Every user session gets assigned a random IP address | Geolocation lookups  |
| User Agent | Every user session gets assigned a random User-Agent | User agent parsing   |
| Referrer   | TODO: Always null                                    | Referrer parsing     |

Some properties like the pageview URL are static because they undergo minimal processing and there is no caching involved.

Each user session generates a call to `/api/event` at random intervals between 10 and 60 seconds. The test aims to
find the maximum number of concurrent users that a given Plausible Analytics server can handle.

### Running

First, make sure to increase the limit of open files and ephemeral ports on the load generating machine.
The following setup will be sufficient to simulate up to 60k simultaneous users.

```bash
$ sudo ulimit -n 100000
$ sudo sysctl -w net.ipv4.ip_local_port_range="1024 65535"
```

Simulating 60k concurrent visitors from one host requires ~45GB of memory.
