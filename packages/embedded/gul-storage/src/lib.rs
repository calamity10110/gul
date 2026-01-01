use anyhow::Result;
#[cfg(target_os = "espidf")]
use embedded_svc::storage::Storage;
#[cfg(target_os = "espidf")]
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsDefault};
use std::sync::{Arc, Mutex};

pub struct NvsStorage {
    #[cfg(target_os = "espidf")]
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    #[cfg(not(target_os = "espidf"))]
    nvs: Arc<Mutex<()>>, // Stub field
}

impl NvsStorage {
    pub fn new(namespace: &str) -> Result<Self> {
        #[cfg(target_os = "espidf")]
        {
            let nvs_default_partition = EspNvsPartition::<NvsDefault>::take()?;
            let nvs = EspNvs::new(nvs_default_partition, namespace, true)?;
            Ok(Self {
                nvs: Arc::new(Mutex::new(nvs)),
            })
        }
        #[cfg(not(target_os = "espidf"))]
        {
            Ok(Self {
                nvs: Arc::new(Mutex::new(())),
            })
        }
    }

    pub fn set_string(&self, key: &str, value: &str) -> Result<()> {
        #[cfg(target_os = "espidf")]
        {
            let mut nvs = self.nvs.lock().unwrap();
            nvs.set_str(key, value)?;
        }
        Ok(())
    }

    pub fn get_string(&self, key: &str) -> Result<String> {
        #[cfg(target_os = "espidf")]
        {
            let nvs = self.nvs.lock().unwrap();
            // Fixed buffer size for credentials
            let mut buf = [0u8; 64];
            let val = nvs.get_str(key, &mut buf)?;
            match val {
                Some(s) => Ok(s.to_string()),
                None => Err(anyhow::anyhow!("Key not found")),
            }
        }
        #[cfg(not(target_os = "espidf"))]
        {
            Err(anyhow::anyhow!("Not implemented on host"))
        }
    }
}

pub struct FsStorage;
impl FsStorage {
    // Stub for SD/SPIFFS/FatFS interactions
    // In real impl: Mount partition, return file handles
    pub fn init_sd_card() -> Result<()> {
        log::info!("Mounting SD Card...");
        Ok(())
    }
}
