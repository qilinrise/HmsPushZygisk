pub const CONFIG_PATH: &str = "/data/adb/hmspush/app.conf";

pub const HMSPUSH_PACKAGE_NAME: &str = "one.yufz.hmspush";

#[derive(Clone, Copy)]
pub struct PackageProps<'a> {
    pub package_name: &'a str,
    pub system_properties: &'a [(&'a str, &'a str)],
    pub build_properties: &'a [(&'a str, &'a str)],
}

pub const DEFAULT_PACKAGE_PROPS: PackageProps<'static> = PackageProps {
    package_name: "",
    system_properties: &[
        ("ro.build.version.emui", "EmotionUI_8.0.0"),
        ("ro.build.hw_emui_api_level", "21"),
    ],
    build_properties: &[("BRAND", "Huawei"), ("MANUFACTURER", "HUAWEI")],
};

// 百度贴吧专属：HUAWEI Pura 70 Pro (HBN-AL00) 伪装配置
const TIEBA_PURA_70_PRO_PROPS: PackageProps<'static> = PackageProps {
    package_name: "com.baidu.tieba",
    system_properties: &[
        ("ro.build.product", "HBN-AL00"),
        ("ro.product.brand", "HUAWEI"),
        ("ro.product.device", "HBN-AL00"),
        ("ro.product.manufacturer", "HUAWEI"),
        ("ro.product.model", "HBN-AL00"),
        ("ro.product.name", "HBN-AL00"),
        ("ro.product.marketname", "HUAWEI Pura 70 Pro"),
        ("ro.build.version.emui", "EMUI13"),
        ("ro.build.hw_emui_api_level", "29"),
    ],
    build_properties: &[
        ("BRAND", "HUAWEI"),
        ("MANUFACTURER", "HUAWEI"),
        ("MODEL", "HBN-AL00"),
        ("PRODUCT", "HBN-AL00"),
        ("DEVICE", "HBN-AL00"),
    ],
};

pub const PACKAGE_PROPS: &[PackageProps] = &[
    PackageProps {
        package_name: HMSPUSH_PACKAGE_NAME,
        system_properties: &[("hmspush.zygisk.enabled", "true")],
        build_properties: &[],
    },
    PackageProps {
        package_name: "com.sankuai.meituan",
        system_properties: &[("ro.build.version.emui", "EmotionUI_8.0.0")],
        build_properties: &[],
    },
    PackageProps {
        package_name: "com.sankuai.meituan.takeoutnew",
        system_properties: &[("ro.build.version.emui", "EmotionUI_8.0.0")],
        build_properties: &[],
    },
    PackageProps {
        package_name: "com.dianping.v1",
        system_properties: &[("ro.build.version.emui", "EmotionUI_8.0.0")],
        build_properties: &[],
    },
    PackageProps {
        package_name: "com.tencent.mobileqq",
        system_properties: &[],
        build_properties: &[("MANUFACTURER", "HUAWEI")],
    },
    PackageProps {
        package_name: "tv.danmaku.bili",
        system_properties: &[],
        build_properties: &[("MANUFACTURER", "HUAWEI")],
    },
    PackageProps {
        package_name: "com.xunmeng.pinduoduo",
        system_properties: &[],
        build_properties: &[("MANUFACTURER", "HUAWEI")],
    },
    TIEBA_PURA_70_PRO_PROPS,
];

#[inline]
pub fn get_properties_for_package(pkg: &str) -> PackageProps<'static> {
    PACKAGE_PROPS
        .iter()
        .find(|p| p.package_name == pkg)
        .copied()
        .unwrap_or(DEFAULT_PACKAGE_PROPS)
}
