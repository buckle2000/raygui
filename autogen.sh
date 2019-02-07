#!/bin/bash

pushd autogen && python3 gen.py src > bindings.rs && popd && exit
exit 1