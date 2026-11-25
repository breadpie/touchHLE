/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `GKLocalPlayer`.

use crate::dyld::{ConstantExports, HostConstant};
use crate::objc::{id, nil, objc_classes, ClassExports, NSZonePtr};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// TODO: proper inheritance chain
@implementation GKLocalPlayer: NSObject

+ (id)localPlayer {
    // TODO
    nil
}

@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation GKAchievement: NSObject
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation GKAchievementViewController: UIViewController
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation GKLeaderboardViewController: UIViewController
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation NSNumberFormatter: NSObject
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation UINavigationBar: UIView
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation UINavigationItem: NSObject
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation UIBarButtonItem: NSObject
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

// Stub: creating one returns nil, so every message sent to it is ignored.
@implementation UITabBarController: UIViewController
+ (id)allocWithZone:(NSZonePtr)_zone {
    nil
}
@end

@implementation NSAssertionHandler: NSObject
+ (id)currentHandler {
    nil
}
@end

};

pub const GKPlayerAuthenticationDidChangeNotificationName: &str =
    "GKPlayerAuthenticationDidChangeNotificationName";

/// `NSNotificationName` values.
pub const CONSTANTS: ConstantExports = &[(
    "_GKPlayerAuthenticationDidChangeNotificationName",
    HostConstant::NSString(GKPlayerAuthenticationDidChangeNotificationName),
)];
