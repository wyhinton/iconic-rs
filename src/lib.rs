//! This crate contains the auto-generated mapping of icon name for several popular icon fonts.
//! This crate does not provide the fonts or svg's themselves, but provides eponymous pub constants for  
//! (such as `Icon::NetworkWifi`) to the character codepoint `\u{e1ba}`)
//! in Googles Material Icon Font ([https://material.io/tools/icons/](https://material.io/tools/icons/)) -
//! useful if you want to use the material-icons font in user interfaces created
//! in Rust
//!
//! ## Example
//!
//! ```rust
// ! use material_icons::{Icon, icon_to_char};
// ! let icon_char = icon_to_char(Icon::Rotation3d);
// ! assert_eq!('\u{e84d}', icon_char);
//! ```
//!
//! ## License (please read - regarding embedded font)
/// Octicons Icons


#[allow(dead_code)]
pub struct Octicons;
#[allow(dead_code)]
impl Octicons {
/// ## [heart](https://github.com/primer/octicons/blob/main/icons/heart-16.svg)

	pub const HEART: u32 = 9829;
/// ## [zap](https://github.com/primer/octicons/blob/main/icons/zap-16.svg)

	pub const ZAP: u32 = 9889;
/// ## [light-bulb](https://github.com/primer/octicons/blob/main/icons/light-bulb-16.svg)

	pub const LIGHTBULB: u32 = 61440;
/// ## [repo](https://github.com/primer/octicons/blob/main/icons/repo-16.svg)

	pub const REPO: u32 = 61441;
/// ## [repo-forked](https://github.com/primer/octicons/blob/main/icons/repo-forked-16.svg)

	pub const REPOFORKED: u32 = 61442;
/// ## [repo-push](https://github.com/primer/octicons/blob/main/icons/repo-push-16.svg)

	pub const REPOPUSH: u32 = 61445;
/// ## [repo-pull](https://github.com/primer/octicons/blob/main/icons/repo-pull-16.svg)

	pub const REPOPULL: u32 = 61446;
/// ## [book](https://github.com/primer/octicons/blob/main/icons/book-16.svg)

	pub const BOOK: u32 = 61447;
	pub const OCTOFACE: u32 = 61448;
/// ## [git-pull-request](https://github.com/primer/octicons/blob/main/icons/git-pull-request-16.svg)

	pub const GITPULLREQUEST: u32 = 61449;
/// ## [mark-github](https://github.com/primer/octicons/blob/main/icons/mark-github-16.svg)

	pub const MARKGITHUB: u32 = 61450;
	pub const CLOUDDOWNLOAD: u32 = 61451;
	pub const CLOUDUPLOAD: u32 = 61452;
	pub const KEYBOARD: u32 = 61453;
	pub const GIST: u32 = 61454;
/// ## [file-code](https://github.com/primer/octicons/blob/main/icons/file-code-16.svg)

	pub const FILECODE: u32 = 61456;
	pub const FILETEXT: u32 = 61457;
	pub const FILEMEDIA: u32 = 61458;
/// ## [file-zip](https://github.com/primer/octicons/blob/main/icons/file-zip-16.svg)

	pub const FILEZIP: u32 = 61459;
	pub const FILEPDF: u32 = 61460;
/// ## [tag](https://github.com/primer/octicons/blob/main/icons/tag-16.svg)

	pub const TAG: u32 = 61461;
/// ## [file-directory](https://github.com/primer/octicons/blob/main/icons/file-directory-16.svg)

	pub const FILEDIRECTORY: u32 = 61462;
/// ## [file-submodule](https://github.com/primer/octicons/blob/main/icons/file-submodule-16.svg)

	pub const FILESUBMODULE: u32 = 61463;
/// ## [person](https://github.com/primer/octicons/blob/main/icons/person-16.svg)

	pub const PERSON: u32 = 61464;
	pub const JERSEY: u32 = 61465;
/// ## [git-commit](https://github.com/primer/octicons/blob/main/icons/git-commit-16.svg)

	pub const GITCOMMIT: u32 = 61471;
/// ## [git-branch](https://github.com/primer/octicons/blob/main/icons/git-branch-16.svg)

	pub const GITBRANCH: u32 = 61472;
/// ## [git-merge](https://github.com/primer/octicons/blob/main/icons/git-merge-16.svg)

	pub const GITMERGE: u32 = 61475;
/// ## [mirror](https://github.com/primer/octicons/blob/main/icons/mirror-16.svg)

	pub const MIRROR: u32 = 61476;
/// ## [issue-opened](https://github.com/primer/octicons/blob/main/icons/issue-opened-16.svg)

	pub const ISSUEOPENED: u32 = 61478;
/// ## [issue-reopened](https://github.com/primer/octicons/blob/main/icons/issue-reopened-16.svg)

	pub const ISSUEREOPENED: u32 = 61479;
/// ## [issue-closed](https://github.com/primer/octicons/blob/main/icons/issue-closed-16.svg)

	pub const ISSUECLOSED: u32 = 61480;
/// ## [star](https://github.com/primer/octicons/blob/main/icons/star-16.svg)

	pub const STAR: u32 = 61482;
/// ## [comment](https://github.com/primer/octicons/blob/main/icons/comment-16.svg)

	pub const COMMENT: u32 = 61483;
/// ## [question](https://github.com/primer/octicons/blob/main/icons/question-16.svg)

	pub const QUESTION: u32 = 61484;
/// ## [alert](https://github.com/primer/octicons/blob/main/icons/alert-16.svg)

	pub const ALERT: u32 = 61485;
/// ## [search](https://github.com/primer/octicons/blob/main/icons/search-16.svg)

	pub const SEARCH: u32 = 61486;
/// ## [gear](https://github.com/primer/octicons/blob/main/icons/gear-16.svg)

	pub const GEAR: u32 = 61487;
	pub const RADIOTOWER: u32 = 61488;
/// ## [tools](https://github.com/primer/octicons/blob/main/icons/tools-16.svg)

	pub const TOOLS: u32 = 61489;
/// ## [sign-out](https://github.com/primer/octicons/blob/main/icons/sign-out-16.svg)

	pub const SIGNOUT: u32 = 61490;
/// ## [rocket](https://github.com/primer/octicons/blob/main/icons/rocket-16.svg)

	pub const ROCKET: u32 = 61491;
/// ## [rss](https://github.com/primer/octicons/blob/main/icons/rss-16.svg)

	pub const RSS: u32 = 61492;
/// ## [clippy](https://github.com/primer/octicons/blob/main/icons/clippy-16.svg)

	pub const CLIPPY: u32 = 61493;
/// ## [sign-in](https://github.com/primer/octicons/blob/main/icons/sign-in-16.svg)

	pub const SIGNIN: u32 = 61494;
/// ## [organization](https://github.com/primer/octicons/blob/main/icons/organization-16.svg)

	pub const ORGANIZATION: u32 = 61495;
/// ## [device-mobile](https://github.com/primer/octicons/blob/main/icons/device-mobile-16.svg)

	pub const DEVICEMOBILE: u32 = 61496;
/// ## [unfold](https://github.com/primer/octicons/blob/main/icons/unfold-16.svg)

	pub const UNFOLD: u32 = 61497;
/// ## [check](https://github.com/primer/octicons/blob/main/icons/check-16.svg)

	pub const CHECK: u32 = 61498;
/// ## [mail](https://github.com/primer/octicons/blob/main/icons/mail-16.svg)

	pub const MAIL: u32 = 61499;
	pub const MAILREAD: u32 = 61500;
/// ## [arrow-up](https://github.com/primer/octicons/blob/main/icons/arrow-up-16.svg)

	pub const ARROWUP: u32 = 61501;
/// ## [arrow-right](https://github.com/primer/octicons/blob/main/icons/arrow-right-16.svg)

	pub const ARROWRIGHT: u32 = 61502;
/// ## [arrow-down](https://github.com/primer/octicons/blob/main/icons/arrow-down-16.svg)

	pub const ARROWDOWN: u32 = 61503;
/// ## [arrow-left](https://github.com/primer/octicons/blob/main/icons/arrow-left-16.svg)

	pub const ARROWLEFT: u32 = 61504;
/// ## [pin](https://github.com/primer/octicons/blob/main/icons/pin-16.svg)

	pub const PIN: u32 = 61505;
/// ## [gift](https://github.com/primer/octicons/blob/main/icons/gift-16.svg)

	pub const GIFT: u32 = 61506;
/// ## [graph](https://github.com/primer/octicons/blob/main/icons/graph-16.svg)

	pub const GRAPH: u32 = 61507;
/// ## [triangle-left](https://github.com/primer/octicons/blob/main/icons/triangle-left-16.svg)

	pub const TRIANGLELEFT: u32 = 61508;
/// ## [credit-card](https://github.com/primer/octicons/blob/main/icons/credit-card-16.svg)

	pub const CREDITCARD: u32 = 61509;
/// ## [clock](https://github.com/primer/octicons/blob/main/icons/clock-16.svg)

	pub const CLOCK: u32 = 61510;
/// ## [ruby](https://github.com/primer/octicons/blob/main/icons/ruby-16.svg)

	pub const RUBY: u32 = 61511;
/// ## [broadcast](https://github.com/primer/octicons/blob/main/icons/broadcast-16.svg)

	pub const BROADCAST: u32 = 61512;
/// ## [key](https://github.com/primer/octicons/blob/main/icons/key-16.svg)

	pub const KEY: u32 = 61513;
	pub const REPOFORCEPUSH: u32 = 61514;
/// ## [repo-clone](https://github.com/primer/octicons/blob/main/icons/repo-clone-16.svg)

	pub const REPOCLONE: u32 = 61516;
/// ## [diff](https://github.com/primer/octicons/blob/main/icons/diff-16.svg)

	pub const DIFF: u32 = 61517;
/// ## [eye](https://github.com/primer/octicons/blob/main/icons/eye-16.svg)

	pub const EYE: u32 = 61518;
/// ## [comment-discussion](https://github.com/primer/octicons/blob/main/icons/comment-discussion-16.svg)

	pub const COMMENTDISCUSSION: u32 = 61519;
	pub const MAILREPLY: u32 = 61521;
	pub const PRIMITIVEDOT: u32 = 61522;
	pub const PRIMITIVESQUARE: u32 = 61523;
/// ## [device-camera](https://github.com/primer/octicons/blob/main/icons/device-camera-16.svg)

	pub const DEVICECAMERA: u32 = 61526;
/// ## [device-camera-video](https://github.com/primer/octicons/blob/main/icons/device-camera-video-16.svg)

	pub const DEVICECAMERAVIDEO: u32 = 61527;
/// ## [pencil](https://github.com/primer/octicons/blob/main/icons/pencil-16.svg)

	pub const PENCIL: u32 = 61528;
/// ## [info](https://github.com/primer/octicons/blob/main/icons/info-16.svg)

	pub const INFO: u32 = 61529;
/// ## [triangle-right](https://github.com/primer/octicons/blob/main/icons/triangle-right-16.svg)

	pub const TRIANGLERIGHT: u32 = 61530;
/// ## [triangle-down](https://github.com/primer/octicons/blob/main/icons/triangle-down-16.svg)

	pub const TRIANGLEDOWN: u32 = 61531;
/// ## [link](https://github.com/primer/octicons/blob/main/icons/link-16.svg)

	pub const LINK: u32 = 61532;
/// ## [plus](https://github.com/primer/octicons/blob/main/icons/plus-16.svg)

	pub const PLUS: u32 = 61533;
/// ## [three-bars](https://github.com/primer/octicons/blob/main/icons/three-bars-16.svg)

	pub const THREEBARS: u32 = 61534;
/// ## [code](https://github.com/primer/octicons/blob/main/icons/code-16.svg)

	pub const CODE: u32 = 61535;
/// ## [location](https://github.com/primer/octicons/blob/main/icons/location-16.svg)

	pub const LOCATION: u32 = 61536;
/// ## [list-unordered](https://github.com/primer/octicons/blob/main/icons/list-unordered-16.svg)

	pub const LISTUNORDERED: u32 = 61537;
/// ## [list-ordered](https://github.com/primer/octicons/blob/main/icons/list-ordered-16.svg)

	pub const LISTORDERED: u32 = 61538;
/// ## [quote](https://github.com/primer/octicons/blob/main/icons/quote-16.svg)

	pub const QUOTE: u32 = 61539;
/// ## [versions](https://github.com/primer/octicons/blob/main/icons/versions-16.svg)

	pub const VERSIONS: u32 = 61540;
/// ## [calendar](https://github.com/primer/octicons/blob/main/icons/calendar-16.svg)

	pub const CALENDAR: u32 = 61544;
/// ## [lock](https://github.com/primer/octicons/blob/main/icons/lock-16.svg)

	pub const LOCK: u32 = 61546;
/// ## [diff-added](https://github.com/primer/octicons/blob/main/icons/diff-added-16.svg)

	pub const DIFFADDED: u32 = 61547;
/// ## [diff-removed](https://github.com/primer/octicons/blob/main/icons/diff-removed-16.svg)

	pub const DIFFREMOVED: u32 = 61548;
/// ## [diff-modified](https://github.com/primer/octicons/blob/main/icons/diff-modified-16.svg)

	pub const DIFFMODIFIED: u32 = 61549;
/// ## [diff-renamed](https://github.com/primer/octicons/blob/main/icons/diff-renamed-16.svg)

	pub const DIFFRENAMED: u32 = 61550;
/// ## [horizontal-rule](https://github.com/primer/octicons/blob/main/icons/horizontal-rule-16.svg)

	pub const HORIZONTALRULE: u32 = 61552;
	pub const ARROWSMALLRIGHT: u32 = 61553;
/// ## [milestone](https://github.com/primer/octicons/blob/main/icons/milestone-16.svg)

	pub const MILESTONE: u32 = 61557;
/// ## [checklist](https://github.com/primer/octicons/blob/main/icons/checklist-16.svg)

	pub const CHECKLIST: u32 = 61558;
/// ## [megaphone](https://github.com/primer/octicons/blob/main/icons/megaphone-16.svg)

	pub const MEGAPHONE: u32 = 61559;
/// ## [chevron-right](https://github.com/primer/octicons/blob/main/icons/chevron-right-16.svg)

	pub const CHEVRONRIGHT: u32 = 61560;
/// ## [bookmark](https://github.com/primer/octicons/blob/main/icons/bookmark-16.svg)

	pub const BOOKMARK: u32 = 61563;
	pub const SETTINGS: u32 = 61564;
	pub const DASHBOARD: u32 = 61565;
/// ## [history](https://github.com/primer/octicons/blob/main/icons/history-16.svg)

	pub const HISTORY: u32 = 61566;
/// ## [link-external](https://github.com/primer/octicons/blob/main/icons/link-external-16.svg)

	pub const LINKEXTERNAL: u32 = 61567;
/// ## [mute](https://github.com/primer/octicons/blob/main/icons/mute-16.svg)

	pub const MUTE: u32 = 61568;
/// ## [x](https://github.com/primer/octicons/blob/main/icons/x-16.svg)

	pub const X: u32 = 61569;
/// ## [circle-slash](https://github.com/primer/octicons/blob/main/icons/circle-slash-16.svg)

	pub const CIRCLESLASH: u32 = 61572;
/// ## [pulse](https://github.com/primer/octicons/blob/main/icons/pulse-16.svg)

	pub const PULSE: u32 = 61573;
/// ## [sync](https://github.com/primer/octicons/blob/main/icons/sync-16.svg)

	pub const SYNC: u32 = 61575;
/// ## [telescope](https://github.com/primer/octicons/blob/main/icons/telescope-16.svg)

	pub const TELESCOPE: u32 = 61576;
	pub const GISTSECRET: u32 = 61580;
/// ## [home](https://github.com/primer/octicons/blob/main/icons/home-16.svg)

	pub const HOME: u32 = 61581;
/// ## [stop](https://github.com/primer/octicons/blob/main/icons/stop-16.svg)

	pub const STOP: u32 = 61583;
/// ## [bug](https://github.com/primer/octicons/blob/main/icons/bug-16.svg)

	pub const BUG: u32 = 61585;
/// ## [logo-github](https://github.com/primer/octicons/blob/main/icons/logo-github-16.svg)

	pub const LOGOGITHUB: u32 = 61586;
/// ## [file-binary](https://github.com/primer/octicons/blob/main/icons/file-binary-16.svg)

	pub const FILEBINARY: u32 = 61588;
/// ## [database](https://github.com/primer/octicons/blob/main/icons/database-16.svg)

	pub const DATABASE: u32 = 61590;
/// ## [server](https://github.com/primer/octicons/blob/main/icons/server-16.svg)

	pub const SERVER: u32 = 61591;
/// ## [diff-ignored](https://github.com/primer/octicons/blob/main/icons/diff-ignored-16.svg)

	pub const DIFFIGNORED: u32 = 61593;
/// ## [ellipsis](https://github.com/primer/octicons/blob/main/icons/ellipsis-16.svg)

	pub const ELLIPSIS: u32 = 61594;
	pub const NONEWLINE: u32 = 61596;
/// ## [hubot](https://github.com/primer/octicons/blob/main/icons/hubot-16.svg)

	pub const HUBOT: u32 = 61597;
	pub const ARROWSMALLUP: u32 = 61599;
	pub const ARROWSMALLDOWN: u32 = 61600;
	pub const ARROWSMALLLEFT: u32 = 61601;
/// ## [chevron-up](https://github.com/primer/octicons/blob/main/icons/chevron-up-16.svg)

	pub const CHEVRONUP: u32 = 61602;
/// ## [chevron-down](https://github.com/primer/octicons/blob/main/icons/chevron-down-16.svg)

	pub const CHEVRONDOWN: u32 = 61603;
/// ## [chevron-left](https://github.com/primer/octicons/blob/main/icons/chevron-left-16.svg)

	pub const CHEVRONLEFT: u32 = 61604;
/// ## [triangle-up](https://github.com/primer/octicons/blob/main/icons/triangle-up-16.svg)

	pub const TRIANGLEUP: u32 = 61610;
/// ## [git-compare](https://github.com/primer/octicons/blob/main/icons/git-compare-16.svg)

	pub const GITCOMPARE: u32 = 61612;
/// ## [logo-gist](https://github.com/primer/octicons/blob/main/icons/logo-gist-16.svg)

	pub const LOGOGIST: u32 = 61613;
/// ## [file-symlink-file](https://github.com/primer/octicons/blob/main/icons/file-symlink-file-16.svg)

	pub const FILESYMLINKFILE: u32 = 61616;
	pub const FILESYMLINKDIRECTORY: u32 = 61617;
/// ## [squirrel](https://github.com/primer/octicons/blob/main/icons/squirrel-16.svg)

	pub const SQUIRREL: u32 = 61618;
/// ## [globe](https://github.com/primer/octicons/blob/main/icons/globe-16.svg)

	pub const GLOBE: u32 = 61622;
/// ## [unmute](https://github.com/primer/octicons/blob/main/icons/unmute-16.svg)

	pub const UNMUTE: u32 = 61626;
/// ## [mention](https://github.com/primer/octicons/blob/main/icons/mention-16.svg)

	pub const MENTION: u32 = 61630;
/// ## [package](https://github.com/primer/octicons/blob/main/icons/package-16.svg)

	pub const PACKAGE: u32 = 61636;
/// ## [browser](https://github.com/primer/octicons/blob/main/icons/browser-16.svg)

	pub const BROWSER: u32 = 61637;
/// ## [terminal](https://github.com/primer/octicons/blob/main/icons/terminal-16.svg)

	pub const TERMINAL: u32 = 61640;
/// ## [markdown](https://github.com/primer/octicons/blob/main/icons/markdown-16.svg)

	pub const MARKDOWN: u32 = 61641;
/// ## [dash](https://github.com/primer/octicons/blob/main/icons/dash-16.svg)

	pub const DASH: u32 = 61642;
/// ## [fold](https://github.com/primer/octicons/blob/main/icons/fold-16.svg)

	pub const FOLD: u32 = 61644;
/// ## [inbox](https://github.com/primer/octicons/blob/main/icons/inbox-16.svg)

	pub const INBOX: u32 = 61647;
	pub const TRASHCAN: u32 = 61648;
	pub const PAINTCAN: u32 = 61649;
/// ## [flame](https://github.com/primer/octicons/blob/main/icons/flame-16.svg)

	pub const FLAME: u32 = 61650;
/// ## [briefcase](https://github.com/primer/octicons/blob/main/icons/briefcase-16.svg)

	pub const BRIEFCASE: u32 = 61651;
/// ## [plug](https://github.com/primer/octicons/blob/main/icons/plug-16.svg)

	pub const PLUG: u32 = 61652;
	pub const CIRCUITBOARD: u32 = 61654;
/// ## [mortar-board](https://github.com/primer/octicons/blob/main/icons/mortar-board-16.svg)

	pub const MORTARBOARD: u32 = 61655;
/// ## [law](https://github.com/primer/octicons/blob/main/icons/law-16.svg)

	pub const LAW: u32 = 61656;
/// ## [thumbsup](https://github.com/primer/octicons/blob/main/icons/thumbsup-16.svg)

	pub const THUMBSUP: u32 = 61658;
/// ## [thumbsdown](https://github.com/primer/octicons/blob/main/icons/thumbsdown-16.svg)

	pub const THUMBSDOWN: u32 = 61659;
/// ## [desktop-download](https://github.com/primer/octicons/blob/main/icons/desktop-download-16.svg)

	pub const DESKTOPDOWNLOAD: u32 = 61660;
/// ## [beaker](https://github.com/primer/octicons/blob/main/icons/beaker-16.svg)

	pub const BEAKER: u32 = 61661;
/// ## [bell](https://github.com/primer/octicons/blob/main/icons/bell-16.svg)

	pub const BELL: u32 = 61662;
	pub const WATCH: u32 = 61664;
/// ## [shield](https://github.com/primer/octicons/blob/main/icons/shield-16.svg)

	pub const SHIELD: u32 = 61665;
/// ## [bold](https://github.com/primer/octicons/blob/main/icons/bold-16.svg)

	pub const BOLD: u32 = 61666;
	pub const TEXTSIZE: u32 = 61667;
/// ## [italic](https://github.com/primer/octicons/blob/main/icons/italic-16.svg)

	pub const ITALIC: u32 = 61668;
/// ## [tasklist](https://github.com/primer/octicons/blob/main/icons/tasklist-16.svg)

	pub const TASKLIST: u32 = 61669;
/// ## [verified](https://github.com/primer/octicons/blob/main/icons/verified-16.svg)

	pub const VERIFIED: u32 = 61670;
/// ## [smiley](https://github.com/primer/octicons/blob/main/icons/smiley-16.svg)

	pub const SMILEY: u32 = 61671;
/// ## [unverified](https://github.com/primer/octicons/blob/main/icons/unverified-16.svg)

	pub const UNVERIFIED: u32 = 61672;
/// ## [device-desktop](https://github.com/primer/octicons/blob/main/icons/device-desktop-16.svg)

	pub const DEVICEDESKTOP: u32 = 62076;
	pub const ALL_ICONS: [( &'static str, u32); 167] = [("HEART", 9829),("ZAP", 9889),("LIGHTBULB", 61440),("REPO", 61441),("REPOFORKED", 61442),("REPOPUSH", 61445),("REPOPULL", 61446),("BOOK", 61447),("OCTOFACE", 61448),("GITPULLREQUEST", 61449),("MARKGITHUB", 61450),("CLOUDDOWNLOAD", 61451),("CLOUDUPLOAD", 61452),("KEYBOARD", 61453),("GIST", 61454),("FILECODE", 61456),("FILETEXT", 61457),("FILEMEDIA", 61458),("FILEZIP", 61459),("FILEPDF", 61460),("TAG", 61461),("FILEDIRECTORY", 61462),("FILESUBMODULE", 61463),("PERSON", 61464),("JERSEY", 61465),("GITCOMMIT", 61471),("GITBRANCH", 61472),("GITMERGE", 61475),("MIRROR", 61476),("ISSUEOPENED", 61478),("ISSUEREOPENED", 61479),("ISSUECLOSED", 61480),("STAR", 61482),("COMMENT", 61483),("QUESTION", 61484),("ALERT", 61485),("SEARCH", 61486),("GEAR", 61487),("RADIOTOWER", 61488),("TOOLS", 61489),("SIGNOUT", 61490),("ROCKET", 61491),("RSS", 61492),("CLIPPY", 61493),("SIGNIN", 61494),("ORGANIZATION", 61495),("DEVICEMOBILE", 61496),("UNFOLD", 61497),("CHECK", 61498),("MAIL", 61499),("MAILREAD", 61500),("ARROWUP", 61501),("ARROWRIGHT", 61502),("ARROWDOWN", 61503),("ARROWLEFT", 61504),("PIN", 61505),("GIFT", 61506),("GRAPH", 61507),("TRIANGLELEFT", 61508),("CREDITCARD", 61509),("CLOCK", 61510),("RUBY", 61511),("BROADCAST", 61512),("KEY", 61513),("REPOFORCEPUSH", 61514),("REPOCLONE", 61516),("DIFF", 61517),("EYE", 61518),("COMMENTDISCUSSION", 61519),("MAILREPLY", 61521),("PRIMITIVEDOT", 61522),("PRIMITIVESQUARE", 61523),("DEVICECAMERA", 61526),("DEVICECAMERAVIDEO", 61527),("PENCIL", 61528),("INFO", 61529),("TRIANGLERIGHT", 61530),("TRIANGLEDOWN", 61531),("LINK", 61532),("PLUS", 61533),("THREEBARS", 61534),("CODE", 61535),("LOCATION", 61536),("LISTUNORDERED", 61537),("LISTORDERED", 61538),("QUOTE", 61539),("VERSIONS", 61540),("CALENDAR", 61544),("LOCK", 61546),("DIFFADDED", 61547),("DIFFREMOVED", 61548),("DIFFMODIFIED", 61549),("DIFFRENAMED", 61550),("HORIZONTALRULE", 61552),("ARROWSMALLRIGHT", 61553),("MILESTONE", 61557),("CHECKLIST", 61558),("MEGAPHONE", 61559),("CHEVRONRIGHT", 61560),("BOOKMARK", 61563),("SETTINGS", 61564),("DASHBOARD", 61565),("HISTORY", 61566),("LINKEXTERNAL", 61567),("MUTE", 61568),("X", 61569),("CIRCLESLASH", 61572),("PULSE", 61573),("SYNC", 61575),("TELESCOPE", 61576),("GISTSECRET", 61580),("HOME", 61581),("STOP", 61583),("BUG", 61585),("LOGOGITHUB", 61586),("FILEBINARY", 61588),("DATABASE", 61590),("SERVER", 61591),("DIFFIGNORED", 61593),("ELLIPSIS", 61594),("NONEWLINE", 61596),("HUBOT", 61597),("ARROWSMALLUP", 61599),("ARROWSMALLDOWN", 61600),("ARROWSMALLLEFT", 61601),("CHEVRONUP", 61602),("CHEVRONDOWN", 61603),("CHEVRONLEFT", 61604),("TRIANGLEUP", 61610),("GITCOMPARE", 61612),("LOGOGIST", 61613),("FILESYMLINKFILE", 61616),("FILESYMLINKDIRECTORY", 61617),("SQUIRREL", 61618),("GLOBE", 61622),("UNMUTE", 61626),("MENTION", 61630),("PACKAGE", 61636),("BROWSER", 61637),("TERMINAL", 61640),("MARKDOWN", 61641),("DASH", 61642),("FOLD", 61644),("INBOX", 61647),("TRASHCAN", 61648),("PAINTCAN", 61649),("FLAME", 61650),("BRIEFCASE", 61651),("PLUG", 61652),("CIRCUITBOARD", 61654),("MORTARBOARD", 61655),("LAW", 61656),("THUMBSUP", 61658),("THUMBSDOWN", 61659),("DESKTOPDOWNLOAD", 61660),("BEAKER", 61661),("BELL", 61662),("WATCH", 61664),("SHIELD", 61665),("BOLD", 61666),("TEXTSIZE", 61667),("ITALIC", 61668),("TASKLIST", 61669),("VERIFIED", 61670),("SMILEY", 61671),("UNVERIFIED", 61672),("DEVICEDESKTOP", 62076),];
}