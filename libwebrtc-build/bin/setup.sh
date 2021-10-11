if [ ! -d libwebrtc/depot_tools ];
then
    git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git libwebrtc/depot_tools
fi

export PATH=$PATH:$PWD/libwebrtc/depot_tools