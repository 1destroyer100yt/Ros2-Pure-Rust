# Interop scenarios

Each script here runs against stock ROS 2 inside the image built from `Dockerfile`, and
prints exactly one `PASS:` or `FAIL:` line. CI runs them for every DDS vendor
(`.github/workflows/interop.yml`).

## Running locally

```sh
# Once per container: start Docker if it is not running (see docs/notes/environment.md).
dockerd > /tmp/dockerd.log 2>&1 &

docker build -t prr-interop tests/interop
docker run --rm -e RMW_IMPLEMENTATION=rmw_cyclonedds_cpp \
  -v "$PWD/tests/interop:/interop:ro" prr-interop /interop/smoke.sh
```

## Scenarios

| Script | Proves |
|---|---|
| `smoke.sh` | The harness works: a stock C++ talker is discovered and its messages are received with `ros2 topic echo`. |
