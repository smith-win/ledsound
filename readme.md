# Sound Visuals in Rust

Rust program listen for sund output and make visuals, 
ultimate goal being LED Matrix Display on my Raspberry PI
music player.


## Current Approach
## Enabling Faster Dev on Desktop

### Visuals
Use the Rust lib for led-matrix, see if there is another "Draw" impl to screen so I can practise ??

Yes!!, there is [https://crates.io/crates/embedded-graphics-simulator]

See examples here on updating:  [https://github.com/rust-rpi-led-matrix/rust-rpi-rgb-led-matrix]

#### 
Got error running
```
  = note: /usr/bin/ld: cannot find -lSDL2
          collect2: error: ld returned 1 exit status
```
Tryied this, not fedora
```
sudo dnf install libsdl2-dev
```
Looked on pkgs.org, then tried, 
```
sudo dnf install SDL2-devel
```
And it worked !!!!


### Sound
Hound (see below) did not like PCM .. see if I can get a PCM stream from Pipewire in exactly same format,
ans use info as per Python project to get basic rendering

# Ideas
Spectrum - bars (16, each 2 wide) bass on left, treble on right, 
Flow - a level that scrolls from right to the left and shows peak noise level of music


[https://swharden.com/blog/2009-06-19-reading-pcm-audio-with-python/]

-- this means what ???
"44100:16:1"   44100 Hz, 16 Bits, 2 channels  (samplerate, bits, channels .. prob just L&R)
channels -

(we could set channels to 1 - and ignore stero effect, and bits to 8?) - prob a good first step

We will just get 2 bytes .. we can update screen every 1/4 of second to begin with ??
--> experiment by reading for 10s and see how many bytes there are ??


[http://billposer.org/Linguistics/Computation/LectureNotes/AudioData.html]
>> it is necessary to know their sampling rate, resolution, signedness, and number of channe


[https://stackoverflow.com/questions/49764773/how-is-audio-data-stored-in-raw-pcm-format]
-- seems we can get the data from there

[https://wiki.multimedia.cx/index.php/PCM#PCM_Parameters]



Linux Sound ...
    ALSA
    PulseAudio
    PipeWire (has compatibilty layer with puse)

"PulseAudio basically sits atop ALSA, and use it internally. ALSA is unable by itself to be used by multiple applications, so PulseAudio provides this functionality among others."

https://askubuntu.com/questions/581128/what-is-the-relation-between-alsa-and-pulseaudio-sound-architecture

Option 1
https://unix.stackexchange.com/questions/532042/how-can-i-grab-pulseaudio-output


08Sep22 Pulse would seem to be good option

https://crates.io/crates/libpulse-sys
https://crates.io/crates/libpulse-binding


OR

Get MPD to do the donkey work ...

https://www.runeaudio.com/forum/add-audio-output-fifo-to-mpd-conf-t6329.html

then read fifo file .. need to xform the PCM > which is where "hound" comes in

https://stackoverflow.com/questions/17416112/apply-fft-on-pcm-data-and-convert-to-a-spectrogram
https://crates.io/crates/rustfft



## Fedora Desktop - fifo

[https://gitlab.freedesktop.org/pipewire/pipewire/-/issues/934]

```
pactl load-module module-null-sink sink_name=Snapcast object.linger=1 media.class=Audio/Sink format=u8
```
response: 536870913
(not sure if reliable, re, survives restart or what .. how do i delete it ???)
use that on "pactl unload module 536870913'


```
mkfifo /tmp/stuart_fifo
```

```
parec -d Snapcast > /tmp/stuart_fifo
```

Hound did not read >> FormatError("no RIFF tag found")
-- may need to "assume" the format, perhap another library ??? (dasp)?

-- this now hangs  .. what is volume control saying ???

Volume control now shows this ... play something ???




