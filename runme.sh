#!/bin/bash

cargo build

export GST_DEBUG="2,GST_BUFFER:7"
export GST_PLUGIN_PATH="target/debug/"
gst-launch-1.0 videotestsrc num-buffers=2 ! myfilter ! fakesink
