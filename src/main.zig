const std = @import("std");
const c = @import("./c.zig");
const xerror = @import("./xerror.zig");

pub fn main() !void {
    var dpy = c.XOpenDisplay(null) orelse return error.CannotOpenDisplay;
    defer _ = c.XCloseDisplay(dpy);

    checkOtherWM(dpy);
}

fn checkOtherWM(dpy: *c.Display) void {
    xerror.xErrorXlib = c.XSetErrorHandler(xerror.xErrorStart);
    _ = c.XSelectInput(dpy, c.DefaultRootWindow(dpy), c.SubstructureRedirectMask);
    _ = c.XSync(dpy, c.False);
    _ = c.XSetErrorHandler(xerror.xError);
    _ = c.XSync(dpy, c.False);
}
