# Digital-Ocean-Live-Projection
A projector hooked to a webcam pointed at the projection is needed with mouse.  Creates a digital ocean effect.  Requires a four corner calibration before display.  Wrong math made something beautiful so I share.
This is presently PC code, but rust - so there's a good change I can get it to run on a mac or Raspberry pi if I set my mind to it.  And you can probably do just as well as I can.

HOW TO:
[1]  Set the projector as only computer output or as dual mirrored display.
[2]  Align webcam so that it is aimed at and centered on projection.  Webcam view must enclose all of projection and be centered such that projection edges are in NW, NE, SW, SE quadrants of screen.
      you can edit the code if you have multiple webcams, remote webcams. etc.  I have use a logitec webcam and they are my presets, but the interface seems to work with elgato so you can your phone as a webcamera if you have only slightly more additional downloads and patience than I have.  Sorry, um... you are left to your own devices when it comes to getting your webcam to work.  Best luck.
[3]  Compile for rust with --Release flag.  Additionally for highest frame rate probably compile with opt-level=3 and fat LTO (link time optimization) in Cargo toml.
[4]  Run.  The projection will flash and the camera with capture.  
[5]  In tiny 6 point font there will be camera to projector calibration instructions... um... I'm not a "screen font bitmaps are low effort" programming ninja, sorry.  
      Just left click the four corners of the bright projection to calibrate.
[6]  The digital ocean should roll in.  Enjoy the bright effect bad mathematics has on your skin, and bask in the subtle photonic waves.  
[7]  Not much motion in the ocean?  um... you could rewrite the webcam drivers to make it melty or... try a faster webcam, or live feed from your phone with elgato's software?  Or try a beefy computer.  Or remember to use "cargo run --release".
[8[  Resolution?  Yes, hardcoded mentions to a resolution supported by my projector might be included.  Seek them out and change them if required.

PHOTOSENSITIVE WARNING:  The digital ocean may or may not cause drowning deaths during photosensitive seziures.  Just how flashy it gets depents on how much motion is in the webcam and situational lighting.  I find it calming, but if pool caustic lensing, sun flashing through palm leaves, and reflections are a hazard, for your safety, please stay clear.
