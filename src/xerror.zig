const std = @import("std");
const c = @import("./c.zig");

pub var xErrorXlib: c.XErrorHandler = undefined;

pub fn xError(dpy: ?*c.Display, ee: [*c]c.XErrorEvent) callconv(.C) c_int {
    // zig fmt: off
    if (ee.*.error_code == c.BadWindow
        or (ee.*.request_code == c.X_SetInputFocus     and ee.*.error_code == c.BadMatch)
        or (ee.*.request_code == c.X_PolyText8         and ee.*.error_code == c.BadDrawable)
        or (ee.*.request_code == c.X_PolyFillRectangle and ee.*.error_code == c.BadDrawable)
        or (ee.*.request_code == c.X_PolySegment       and ee.*.error_code == c.BadDrawable)
        or (ee.*.request_code == c.X_ConfigureWindow  and ee.*.error_code == c.BadMatch)
        or (ee.*.request_code == c.X_GrabButton        and ee.*.error_code == c.BadAccess)
        or (ee.*.request_code == c.X_GrabKey           and ee.*.error_code == c.BadAccess)
        or (ee.*.request_code == c.X_CopyArea          and ee.*.error_code == c.BadDrawable)) return 0;
    // zig fmt: on

    std.debug.print("FATAL_ERROR: request code:{}, error code:{}\n", .{ ee.*.request_code, ee.*.error_code });
    return xErrorXlib.?(dpy, ee);
}

pub fn xErrorStart(dpy: ?*c.Display, ee: [*c]c.XErrorEvent) callconv(.C) c_int {
    _ = dpy;
    _ = ee;

    std.debug.print("ERROR: another window manager is running\n", .{});
    return -1;
}
