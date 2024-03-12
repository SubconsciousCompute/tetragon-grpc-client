# tetragon-grpc-client


```sh
docker run --name tetragon --rm -d                  \
   --pid=host --cgroupns=host --privileged          \
   -v /sys/kernel/btf/vmlinux:/var/lib/tetragon/btf \
   -v /var/run/tetragon/tetragon.sock:/var/run/tetragon/tetragon.sock \
   -v /home/aditeya/code/tetragon-grpc-client:/tetragon-grpc-client/ \
   -p 54321:54321 \
   quay.io/cilium/tetragon-ci:latest
```
