// Mobile Platform Support for GUL
// Provides Android and iOS build support via WASM, mobile UI components, and native API bridges

use std::collections::HashMap;

/// Mobile platform target
#[derive(Debug, Clone, PartialEq)]
pub enum MobilePlatform {
    Android { api_level: u32, arch: AndroidArch },
    IOS { version: String, arch: IOSArch },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AndroidArch {
    ARM64,
    ARMv7,
    X86_64,
    X86,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IOSArch {
    ARM64,    // iPhone/iPad
    X86_64,   // Simulator
    Arm64Sim, // M1 Mac Simulator
}

/// Mobile build configuration
pub struct MobileBuildConfig {
    pub platform: MobilePlatform,
    pub app_name: String,
    pub bundle_id: String,
    pub version: String,
    pub use_wasm: bool,
    pub native_bridges: Vec<String>,
}

/// Mobile UI component
#[derive(Debug, Clone)]
pub struct MobileUIComponent {
    pub id: String,
    pub component_type: UIComponentType,
    pub properties: HashMap<String, String>,
    pub children: Vec<MobileUIComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UIComponentType {
    View,
    Text,
    Button,
    Image,
    ScrollView,
    ListView,
    TextInput,
    Switch,
    Slider,
    ActivityIndicator,
    Modal,
    SafeAreaView,
    StatusBar,
    TouchableOpacity,
    FlatList,
    SectionList,
}

/// Native API bridge for mobile platforms
pub struct NativeAPIBridge {
    platform: MobilePlatform,
    apis: HashMap<String, NativeAPI>,
}

#[derive(Debug, Clone)]
pub struct NativeAPI {
    pub name: String,
    pub category: APICategory,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum APICategory {
    Camera,
    Location,
    Storage,
    Network,
    Sensors,
    Notifications,
    Permissions,
    FileSystem,
    Contacts,
    Calendar,
    Media,
    Biometrics,
    InAppPurchase,
    Analytics,
}

impl MobileBuildConfig {
    pub fn new_android(app_name: String, bundle_id: String) -> Self {
        MobileBuildConfig {
            platform: MobilePlatform::Android {
                api_level: 33,
                arch: AndroidArch::ARM64,
            },
            app_name,
            bundle_id,
            version: "1.0.0".to_string(),
            use_wasm: true,
            native_bridges: Vec::new(),
        }
    }

    pub fn new_ios(app_name: String, bundle_id: String) -> Self {
        MobileBuildConfig {
            platform: MobilePlatform::IOS {
                version: "16.0".to_string(),
                arch: IOSArch::ARM64,
            },
            app_name,
            bundle_id,
            version: "1.0.0".to_string(),
            use_wasm: true,
            native_bridges: Vec::new(),
        }
    }

    pub fn add_native_bridge(&mut self, bridge: String) {
        self.native_bridges.push(bridge);
    }

    pub fn set_wasm_enabled(&mut self, enabled: bool) {
        self.use_wasm = enabled;
    }

    pub fn generate_build_script(&self) -> String {
        match &self.platform {
            MobilePlatform::Android { .. } => self.generate_android_build(),
            MobilePlatform::IOS { .. } => self.generate_ios_build(),
        }
    }

    fn generate_android_build(&self) -> String {
        let mut script = String::new();
        script.push_str("#!/bin/bash\n");
        script.push_str("# Android build script for GUL\n\n");

        if self.use_wasm {
            script.push_str("# Build WASM module\n");
            script.push_str("cargo build --target wasm32-unknown-unknown --release\n\n");
            script.push_str("# Optimize WASM\n");
            script.push_str(
                "wasm-opt -Oz -o app.wasm target/wasm32-unknown-unknown/release/app.wasm\n\n",
            );
        }

        script.push_str("# Build Android APK\n");
        script.push_str(&format!(
            "./gradlew assembleRelease -PappName={}\n",
            self.app_name
        ));

        script
    }

    fn generate_ios_build(&self) -> String {
        let mut script = String::new();
        script.push_str("#!/bin/bash\n");
        script.push_str("# iOS build script for GUL\n\n");

        if self.use_wasm {
            script.push_str("# Build WASM module\n");
            script.push_str("cargo build --target wasm32-unknown-unknown --release\n\n");
        }

        script.push_str("# Build iOS app\n");
        script.push_str(&format!(
            "xcodebuild -project {}.xcodeproj -scheme {} -configuration Release\n",
            self.app_name, self.app_name
        ));

        script
    }
}

impl MobileUIComponent {
    pub fn new(id: String, component_type: UIComponentType) -> Self {
        MobileUIComponent {
            id,
            component_type,
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    pub fn add_child(&mut self, child: MobileUIComponent) {
        self.children.push(child);
    }

    pub fn to_jsx(&self) -> String {
        let mut jsx = String::new();
        let tag = format!("{:?}", self.component_type);

        jsx.push_str(&format!("<{}", tag));

        // Add properties
        for (key, value) in &self.properties {
            jsx.push_str(&format!(" {}=\"{}\"", key, value));
        }

        if self.children.is_empty() {
            jsx.push_str(" />");
        } else {
            jsx.push_str(">");
            for child in &self.children {
                jsx.push_str(&child.to_jsx());
            }
            jsx.push_str(&format!("</{}>", tag));
        }

        jsx
    }
}

impl NativeAPIBridge {
    pub fn new(platform: MobilePlatform) -> Self {
        let mut bridge = NativeAPIBridge {
            platform: platform.clone(),
            apis: HashMap::new(),
        };
        bridge.register_standard_apis();
        bridge
    }

    fn register_standard_apis(&mut self) {
        // Camera API
        self.apis.insert(
            "Camera".to_string(),
            NativeAPI {
                name: "Camera".to_string(),
                category: APICategory::Camera,
                methods: vec![
                    "takePicture".to_string(),
                    "recordVideo".to_string(),
                    "switchCamera".to_string(),
                ],
            },
        );

        // Location API
        self.apis.insert(
            "Location".to_string(),
            NativeAPI {
                name: "Location".to_string(),
                category: APICategory::Location,
                methods: vec![
                    "getCurrentPosition".to_string(),
                    "watchPosition".to_string(),
                    "clearWatch".to_string(),
                ],
            },
        );

        // Storage API
        self.apis.insert(
            "Storage".to_string(),
            NativeAPI {
                name: "Storage".to_string(),
                category: APICategory::Storage,
                methods: vec![
                    "setItem".to_string(),
                    "getItem".to_string(),
                    "removeItem".to_string(),
                    "clear".to_string(),
                ],
            },
        );

        // Network API
        self.apis.insert(
            "Network".to_string(),
            NativeAPI {
                name: "Network".to_string(),
                category: APICategory::Network,
                methods: vec![
                    "fetch".to_string(),
                    "getNetworkState".to_string(),
                    "addEventListener".to_string(),
                ],
            },
        );

        // Sensors API
        self.apis.insert(
            "Sensors".to_string(),
            NativeAPI {
                name: "Sensors".to_string(),
                category: APICategory::Sensors,
                methods: vec![
                    "getAccelerometer".to_string(),
                    "getGyroscope".to_string(),
                    "getMagnetometer".to_string(),
                ],
            },
        );

        // Notifications API
        self.apis.insert(
            "Notifications".to_string(),
            NativeAPI {
                name: "Notifications".to_string(),
                category: APICategory::Notifications,
                methods: vec![
                    "scheduleNotification".to_string(),
                    "cancelNotification".to_string(),
                    "requestPermission".to_string(),
                ],
            },
        );

        // Biometrics API
        self.apis.insert(
            "Biometrics".to_string(),
            NativeAPI {
                name: "Biometrics".to_string(),
                category: APICategory::Biometrics,
                methods: vec![
                    "authenticate".to_string(),
                    "isAvailable".to_string(),
                    "getSupportedTypes".to_string(),
                ],
            },
        );
    }

    pub fn get_api(&self, name: &str) -> Option<&NativeAPI> {
        self.apis.get(name)
    }

    pub fn has_api(&self, name: &str) -> bool {
        self.apis.contains_key(name)
    }

    pub fn list_apis(&self) -> Vec<&NativeAPI> {
        self.apis.values().collect()
    }

    pub fn generate_bridge_code(&self) -> String {
        let mut code = String::new();

        match &self.platform {
            MobilePlatform::Android { .. } => {
                code.push_str("// Android Native Bridge\n");
                code.push_str("package com.gul.bridge;\n\n");
                code.push_str("import android.webkit.JavascriptInterface;\n\n");
                code.push_str("public class GulBridge {\n");

                for api in self.apis.values() {
                    for method in &api.methods {
                        code.push_str(&format!("    @JavascriptInterface\n"));
                        code.push_str(&format!("    public void {}() {{\n", method));
                        code.push_str("        // Implementation\n");
                        code.push_str("    }\n\n");
                    }
                }

                code.push_str("}\n");
            }
            MobilePlatform::IOS { .. } => {
                code.push_str("// iOS Native Bridge\n");
                code.push_str("import Foundation\n");
                code.push_str("import WebKit\n\n");
                code.push_str("class GulBridge: NSObject, WKScriptMessageHandler {\n");
                code.push_str("    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {\n");
                code.push_str("        // Handle messages from JavaScript\n");
                code.push_str("    }\n");
                code.push_str("}\n");
            }
        }

        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_build_config() {
        let config =
            MobileBuildConfig::new_android("MyApp".to_string(), "com.example.myapp".to_string());

        assert!(matches!(config.platform, MobilePlatform::Android { .. }));
        assert_eq!(config.app_name, "MyApp");
        assert!(config.use_wasm);
    }

    #[test]
    fn test_ios_build_config() {
        let config =
            MobileBuildConfig::new_ios("MyApp".to_string(), "com.example.myapp".to_string());

        assert!(matches!(config.platform, MobilePlatform::IOS { .. }));
        assert_eq!(config.app_name, "MyApp");
    }

    #[test]
    fn test_ui_component() {
        let mut view = MobileUIComponent::new("root".to_string(), UIComponentType::View);
        view.set_property("style".to_string(), "flex: 1".to_string());

        let mut text = MobileUIComponent::new("text1".to_string(), UIComponentType::Text);
        text.set_property("content".to_string(), "Hello World".to_string());

        view.add_child(text);

        let jsx = view.to_jsx();
        assert!(jsx.contains("View"));
        assert!(jsx.contains("Text"));
    }

    #[test]
    fn test_native_api_bridge() {
        let platform = MobilePlatform::Android {
            api_level: 33,
            arch: AndroidArch::ARM64,
        };
        let bridge = NativeAPIBridge::new(platform);

        assert!(bridge.has_api("Camera"));
        assert!(bridge.has_api("Location"));
        assert!(bridge.has_api("Storage"));
        assert!(bridge.has_api("Notifications"));
    }

    #[test]
    fn test_bridge_code_generation() {
        let platform = MobilePlatform::Android {
            api_level: 33,
            arch: AndroidArch::ARM64,
        };
        let bridge = NativeAPIBridge::new(platform);
        let code = bridge.generate_bridge_code();

        assert!(code.contains("package com.gul.bridge"));
        assert!(code.contains("@JavascriptInterface"));
    }

    #[test]
    fn test_build_script_generation() {
        let config =
            MobileBuildConfig::new_android("TestApp".to_string(), "com.test.app".to_string());

        let script = config.generate_build_script();
        assert!(script.contains("cargo build"));
        assert!(script.contains("wasm32-unknown-unknown"));
        assert!(script.contains("gradlew"));
    }
}
