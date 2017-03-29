# Buttplug

[![Build Status](https://travis-ci.org/metafetish/buttplug.svg?branch=master)](https://travis-ci.org/metafetish/buttplug) [![Build status](https://ci.appveyor.com/api/projects/status/g7vtlw95c39in22k?svg=true)](https://ci.appveyor.com/project/qdot/buttplug) [![Patreon donate button](https://img.shields.io/badge/patreon-donate-yellow.svg)](https://www.patreon.com/qdot)

Buttplug is a framework for hooking up hardware to interfaces, where
hardware usually means sex toys, but could honestly be just about
anything. It's basically a userland HID manager for things that may
not specifically be HID.

In more concrete terms, think of Buttplug as something like
[osculator](http://www.osculator.net/) or [VRPN](http://vrpn.org), but
for sex toys. Instead of wiimotes and control surfaces, we interface
with vibrators, electrostim equipment, fucking machines, and other
hardware that can communicate with computers.

The core of buttplug works as a router. It is a Rust based application
that connects to libraries that registers and communicates with
different hardware. Clients can then connect over websockets or
network ports, to claim and interact with the hardware.

## Support The Project

If you find this project helpful, you
can
[support Metafetish projects via Patreon](http://patreon.com/qdot)!
Every donation helps us afford more hardware to reverse, document, and
write code for!

## Supported Hardware

Buttplug supports the following hardware

- Rez Trancevibrator/Drmn Trancevibe/[Harnett Tech USBMC-01](http://www.harnett-tech.com/search.php?act=search&SKU=USBMC-01)
    - Via [libtrancevibe-rs](http://github.com/metafetish/libtrancevibe-rs)
- [Lovense Toys (Max/Nora/Lush/Hush/etc...)](http://www.lovense.com)
    - Via [lovesense-rs](http://github.com/metafetish/lovesense-rs)
- [ErosTek ET-312](http://www.erostek.com)
    - Via [buttshock-rs](http://github.com/metafetish/buttshock-rs)

Planned support for

- [Fleshlight Launch](http://fleshlight.com/launch)
   - Have hardawre and [documentation](http://github.com/metafetish/raunch-docs), need to write Rust library
- [Vorze A10 Cyclone SA](http://vorzeinteractive.com)
   - Have hardawre and [documentation](http://github.com/metafetish/cockblender-docs), need to write Rust library
- [Kiiroo](http://www.kiiroo.com)
   - Have hardware and [documentation](http://github.com/metafetish/miiyoo-docs), need to write Rust library
- [Minna KGoal](http://www.minnalife.com/products/kgoal)
   - Have hardware, [python](http://github.com/metafetish/khole-py) and [javascript](http://github.com/metafetish/khole-js) libraries, need to document and write Rust library
- [RealTouch](http://realtouch.com)
   - Have hardware and [python/c library](http://github.com/metafetish/librealtouch), need to port to Rust
- [VStroker](http://vstroker.com)
   - Have hardware and [python library](http://github.com/metafetish/libvstroker), need to port to Rust
- [Minna KGoal](http://www.minnalife.com/products/kgoal)
   - Have hardware, [python library](http://github.com/metafetish/khole-py) and [javascript library](http://github.com/metafetish/khole-js), need to port to Rust
- [WeVibe](http://wevibe.com)
   - Have hardware, need to document protocol and write Rust library
- [Vibease](http://vibease.com)
   - Have hardware, need to reverse engineer
- [ErosTek ET-232](http://www.erostek.com)
   - Need hardware, though may be added to [buttshock-rs](http://github.com/metafetish/buttshock-rs)
- [Estim Systems 2b](http://e-stim.co.uk)
   - Need hardware, though may be added to [buttshock-rs](http://github.com/metafetish/buttshock-rs)
- [Vibratissimo](http://www.vibratissimo.com)
   - Need hardware

## License

Buttplug is BSD licensed.

    Copyright (c) 2016, Metafetish
    All rights reserved.
    
    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:
    
    * Redistributions of source code must retain the above copyright notice, this
      list of conditions and the following disclaimer.
    
    * Redistributions in binary form must reproduce the above copyright notice,
      this list of conditions and the following disclaimer in the documentation
      and/or other materials provided with the distribution.
    
    * Neither the name of buttplug nor the names of its
      contributors may be used to endorse or promote products derived from
      this software without specific prior written permission.
    
    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
    OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
