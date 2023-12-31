# Digital-Ocean-Live-Projection
A projector hooked to a webcam pointed at the projection is needed with mouse.  Creates a digital ocean effect.  Requires a four corner calibration before display.  Wrong math made something beautiful so I share.
This is presently PC code, but rust - so there's a good change I can get it to run on a mac or Raspberry pi if I set my mind to it.  And you can probably do just as well as I can.

HOW TO:
[1]  Set the projector as only computer output or as dual mirrored display.
[2]  Align webcam so that it is aimed at and centered on projection.  Webcam view must enclose all of projection and be centered such that projection edges are in NW, NE, SW, SE quadrants of screen.
[3]  Compile for rust with --Release flag.  Additionally for highest frame rate probably compile with opt-level=3 and fat LTO (link time optimization) in Cargo toml.
[4]  Run.  The projection will flash and the camera with capture.  
[5]  In tiny 6 point font there will be camera to projector calibration instructions... um... still learning how to make font bitmaps work, sorry.  
      Just left click the four corners of the bright projection to calibrate.
[6]  The digital ocean should roll in.  Enjoy the bright effect bad mathematics has on your skin, and bask in the subtle photonic waves.  

PHOTOSENSITIVE WARNING:  The digital ocean may or may not cause drowning deaths during photosensitive seziures.  Just how flashy it gets depents on how much motion is in the webcam and situational lighting.  I find it calming, but if pool caustic lensing, flashing sun through palm leaves, and reflections are a hazard, for your safety, please stay clear.
