Known deficiencies (highest priority listed first):
- The resolution cannot be specified.
  Next steps:
  - Add the resolution as a field to the world file format. Bump the version
    number. Document and attempt to enforce the world file format's
    compatibility, much like you did for the raytracer.
  - Allow the user to specify the resolution at the command line.
  - Handle resizing the window gracefully.

- Saving to bmps and ppms are handled so differently that it is a pain to
  explain to the user whenever they are saved while the image is still being
  drawn. I *think* I covered warning the user whenever that's at play, at least
  broadly, but I know if you try to quit in the middle of drawing *that* message
  doesn't explain it that well.
  Next steps:
  - Fix the dialog box for quitting in the middle of drawing (documented with a
    TODO item in the code).
  - Ideally, make the behavior of saving each more uniform. Of course,
    communicate this to the user (it should be easier!).

- Specifying an path with an invalid extension (neither bmp nor ppm) on the
  command line does not result in an error. In all other cases, it does.
  Specifying a path that will not work as the destination for the file on the
  command line also does not result in an error until the saving is actually
  attempted, which may be significantly after the program is invoked if the
  program is configured to draw slowly in any way. (As of this writing, the
  program can be configured to be slow with the `leisurely-drawing` and/or
  `step-by-step-curves-and-lines` cargo features.)
  Next steps:
  - Attempt to touch the file at the earliest convenience. Handle the error when
    doing that, and when actually writing the image. Weep a little about the
    unpredictability of interacting with the filesystem. Get over yourself.

- Much of the code in
  [src/user_interaction_helpers.rs](src/user_interaction_helpers.rs) is ugly and
  poorly tested.
  Next steps:
  - Test it... I guess. This is not my priority for the project, really, and
    it's been very tedious just *writing* it.
  - Try to structure it better? I doubt there's a silver bullet, but I'll ask
    around for ways to make it cleaner. This bit I am curious about.

- The program doesn't handle saving to multiple images at once.
  Next steps:
  - Implement this so it works from both the command line and the GUI.

- Drawing to the screen involves copying the entire image in my code. This is
  not ideal, and it would be interesting to solve it and attempt to measure it's
  effects. 
  NOTE: Fixing this will likely change how bmps are saved while the
  image is being drawn, perhaps by making it meaningless.
  Next steps:
  - Fix it? There be dragons - mostly in the form of atomics.
  - Attempt to observe any difference in performance caused by fixing it.

- The hot dog color scheme does not appear on macOS or Fedora Asahi Remix 39 (under
  either Wayland or X11). It is unknown on which platforms it does appear.
  Next steps:
  - Weep.
