Run the example:

```shell
./runme.sh
```

The output is: 

```
$ ./runme.sh 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
Setting pipeline to PAUSED ...
Pipeline is PREROLLING ...
0:00:00.019201723 65825 0x5628316f5aa0 ERROR               GST_RUST base_transform.rs:684:<_ as gstreamer_base::subclass::base_transform::BaseTransformImplExt>::parent_propose_allocation::{{closure}}:<filter0> Parent function `propose_allocation` failed
0:00:00.019332462 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:844:gst_buffer_new: new 0x5628319705a0
0:00:00.019345227 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:450:_memory_add: buffer 0x5628319705a0, idx -1, mem 0x7fab9c00cde0
0:00:00.019354053 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:898:gst_buffer_new_allocate: new buffer 0x5628319705a0 of size 115200 from allocator (nil)
0:00:00.019368321 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:1800:gst_buffer_map_range: buffer 0x5628319705a0, idx 0, length -1, flags 0002
0:00:00.019379071 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:244:_get_merged_memory: buffer 0x5628319705a0, idx 0, length 1
0:00:00.020978092 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:844:gst_buffer_new: new 0x5628319706c0
0:00:00.020991097 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:450:_memory_add: buffer 0x5628319706c0, idx -1, mem 0x7fab9c056d00
0:00:00.020998221 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:898:gst_buffer_new_allocate: new buffer 0x5628319706c0 of size 115200 from allocator (nil)
0:00:00.021011526 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:1800:gst_buffer_map_range: buffer 0x5628319706c0, idx 0, length -1, flags 0002
0:00:00.021019351 65825 0x5628316f5aa0 LOG               GST_BUFFER gstbuffer.c:244:_get_merged_memory: buffer 0x5628319706c0, idx 0, length 1
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstSystemClock
Got EOS from element "pipeline0".
Execution ended after 0:00:00.000066838
Setting pipeline to NULL ...
Freeing pipeline ...
```

The issue is that there is no calls to `_gst_buffer_free`. 
