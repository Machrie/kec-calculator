// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 전선 데이터 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableData {
    pub cable_type: String,      // 전선 종류
    pub cores: String,           // 가닥수 (1C, 2C, 3C, 4C)
    pub size: String,            // 규격 (mm²)
    pub quantity: u32,           // 수량
    pub system: String,          // 전압 방식 (1Φ, 3Φ)
    pub ground_wire: String,     // 접지선 (없음, HFIX)
    pub install_method: String,  // 공사방법 (A1, A2, B1, B2, C, D1, E, F)
}

/// 계산 결과 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub total_area: f64,              // 총 단면적 (mm²)
    pub conductor_area: f64,          // 도체 단면적 (mm²)
    pub allowable_current: f64,       // 허용전류 (A)
    pub recommended_conduit: String,  // 추천 전선관 크기
    pub fill_rate: f64,               // 점유율 (%)
    pub install_method_desc: String,  // 공사 방법 설명
}

/// 전선 타입 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableTypeInfo {
    pub code: String,
    pub name: String,
    pub description: String,
    pub max_temp: u32,  // 최고 허용 온도 (°C)
    pub insulation: String,  // 절연체 종류
}

/// KEC IEC 60364-5-52 기준 허용전류 데이터 (2부하/3부하 도체)
/// Table B.52.4 (PVC 70°C), Table B.52.5 (XLPE 90°C)
/// Table B.52.10/B.52.11 (E/F 케이블 트레이)
/// Return: (2 loaded current, 3 loaded current)
fn get_allowable_current_table() -> HashMap<(&'static str, &'static str, &'static str), (f64, f64)> {
    let mut table = HashMap::new();
    
    // ============================================================
    // PVC 절연 (70°C) - IEC 60364-5-52 Table B.52.4
    // 주변온도 30°C (공기) / 20°C (지중) 기준
    // ============================================================
    
    // A1: 단열벽 속 전선관 (단심) - Table B.52.4 Column 2/3
    let pvc_a1 = [
        ("1.5", 14.5, 13.5), ("2.5", 19.5, 18.0), ("4", 26.0, 24.0), ("6", 34.0, 31.0),
        ("10", 46.0, 42.0), ("16", 61.0, 56.0), ("25", 80.0, 73.0), ("35", 99.0, 89.0),
        ("50", 119.0, 108.0), ("70", 151.0, 136.0), ("95", 182.0, 164.0), ("120", 210.0, 188.0),
        ("150", 240.0, 216.0), ("185", 273.0, 245.0), ("240", 321.0, 286.0), ("300", 367.0, 328.0),
        ("400", 424.0, 379.0), ("500", 488.0, 436.0),
    ];
    for (size, c2, c3) in pvc_a1.iter() {
        table.insert((*size, "PVC", "A1"), (*c2, *c3));
    }
    
    // A2: 단열벽 속 전선관 (다심) - Table B.52.4 Column 4/5
    let pvc_a2 = [
        ("1.5", 14.0, 13.0), ("2.5", 18.5, 17.5), ("4", 25.0, 23.0), ("6", 32.0, 29.0),
        ("10", 43.0, 39.0), ("16", 57.0, 52.0), ("25", 75.0, 68.0), ("35", 92.0, 83.0),
        ("50", 110.0, 99.0), ("70", 139.0, 125.0), ("95", 167.0, 150.0), ("120", 192.0, 172.0),
        ("150", 219.0, 196.0), ("185", 248.0, 223.0), ("240", 291.0, 261.0), ("300", 334.0, 298.0),
        ("400", 386.0, 345.0), ("500", 444.0, 397.0),
    ];
    for (size, c2, c3) in pvc_a2.iter() {
        table.insert((*size, "PVC", "A2"), (*c2, *c3));
    }

    // B1: 벽면 고정 전선관 (단심) - Table B.52.4 Column 6/7
    let pvc_b1 = [
        ("1.5", 17.5, 15.5), ("2.5", 24.0, 21.0), ("4", 32.0, 28.0), ("6", 41.0, 36.0),
        ("10", 57.0, 50.0), ("16", 76.0, 68.0), ("25", 101.0, 89.0), ("35", 125.0, 110.0),
        ("50", 151.0, 134.0), ("70", 192.0, 171.0), ("95", 232.0, 207.0), ("120", 269.0, 239.0),
        ("150", 309.0, 275.0), ("185", 353.0, 314.0), ("240", 415.0, 369.0), ("300", 477.0, 423.0),
        ("400", 555.0, 490.0), ("500", 642.0, 565.0),
    ];
    for (size, c2, c3) in pvc_b1.iter() {
        table.insert((*size, "PVC", "B1"), (*c2, *c3));
    }

    // B2: 벽면 고정 전선관 (다심) - Table B.52.4 Column 8/9
    let pvc_b2 = [
        ("1.5", 16.5, 15.0), ("2.5", 23.0, 20.0), ("4", 30.0, 27.0), ("6", 38.0, 34.0),
        ("10", 52.0, 46.0), ("16", 69.0, 62.0), ("25", 90.0, 80.0), ("35", 111.0, 99.0),
        ("50", 133.0, 118.0), ("70", 168.0, 149.0), ("95", 201.0, 179.0), ("120", 232.0, 206.0),
        ("150", 265.0, 236.0), ("185", 300.0, 268.0), ("240", 351.0, 313.0), ("300", 401.0, 358.0),
        ("400", 464.0, 414.0), ("500", 533.0, 476.0),
    ];
    for (size, c2, c3) in pvc_b2.iter() {
        table.insert((*size, "PVC", "B2"), (*c2, *c3));
    }

    // C: 벽면 직접 고정 - Table B.52.4 Column 10/11
    let pvc_c = [
        ("1.5", 19.5, 17.5), ("2.5", 27.0, 24.0), ("4", 36.0, 32.0), ("6", 46.0, 41.0),
        ("10", 63.0, 57.0), ("16", 85.0, 76.0), ("25", 112.0, 96.0), ("35", 138.0, 119.0),
        ("50", 168.0, 144.0), ("70", 213.0, 184.0), ("95", 258.0, 223.0), ("120", 299.0, 259.0),
        ("150", 344.0, 299.0), ("185", 392.0, 341.0), ("240", 461.0, 403.0), ("300", 530.0, 464.0),
        ("400", 614.0, 545.0), ("500", 707.0, 638.0),
    ];
    for (size, c2, c3) in pvc_c.iter() {
        table.insert((*size, "PVC", "C"), (*c2, *c3));
    }

    // D1: 지중 덕트 - Table B.52.4 Column 12/13
    let pvc_d1 = [
        ("1.5", 22.0, 18.0), ("2.5", 29.0, 24.0), ("4", 37.0, 30.0), ("6", 46.0, 38.0),
        ("10", 61.0, 50.0), ("16", 79.0, 64.0), ("25", 101.0, 82.0), ("35", 122.0, 98.0),
        ("50", 144.0, 116.0), ("70", 178.0, 143.0), ("95", 211.0, 169.0), ("120", 240.0, 192.0),
        ("150", 271.0, 217.0), ("185", 304.0, 243.0), ("240", 351.0, 280.0), ("300", 396.0, 316.0),
        ("400", 454.0, 363.0), ("500", 513.0, 410.0),
    ];
    for (size, c2, c3) in pvc_d1.iter() {
        table.insert((*size, "PVC", "D1"), (*c2, *c3));
    }

    // D2: 지중 직매 - Table B.52.4 Column 14/15
    let pvc_d2 = [
        ("1.5", 24.0, 19.0), ("2.5", 32.0, 24.0), ("4", 41.0, 33.0), ("6", 51.0, 41.0),
        ("10", 67.0, 54.0), ("16", 87.0, 70.0), ("25", 112.0, 92.0), ("35", 136.0, 110.0),
        ("50", 161.0, 130.0), ("70", 200.0, 162.0), ("95", 239.0, 193.0), ("120", 273.0, 220.0),
        ("150", 310.0, 246.0), ("185", 349.0, 278.0), ("240", 404.0, 320.0), ("300", 458.0, 359.0),
        ("400", 524.0, 414.0), ("500", 590.0, 467.0),
    ];
    for (size, c2, c3) in pvc_d2.iter() {
        table.insert((*size, "PVC", "D2"), (*c2, *c3));
    }

    // E: 케이블 트레이 다심 (자유 공기 중) - Table B.52.10
    let pvc_e = [
        ("1.5", 22.0, 18.5), ("2.5", 30.0, 25.0), ("4", 40.0, 34.0), ("6", 51.0, 43.0),
        ("10", 70.0, 60.0), ("16", 94.0, 80.0), ("25", 119.0, 101.0), ("35", 148.0, 126.0),
        ("50", 180.0, 153.0), ("70", 232.0, 196.0), ("95", 282.0, 238.0), ("120", 328.0, 276.0),
        ("150", 379.0, 319.0), ("185", 434.0, 364.0), ("240", 514.0, 430.0), ("300", 593.0, 497.0),
        ("400", 694.0, 592.0), ("500", 806.0, 706.0),
    ];
    for (size, c2, c3) in pvc_e.iter() {
        table.insert((*size, "PVC", "E"), (*c2, *c3));
    }
    
    // F: 케이블 트레이 단심 (접촉 배치) - Table B.52.11 (단심 Flat/Touching)
    // 단심 케이블은 다심보다 10-15% 높은 허용전류
    let pvc_f = [
        ("1.5", 25.0, 21.0), ("2.5", 34.0, 28.0), ("4", 45.0, 38.0), ("6", 58.0, 48.0),
        ("10", 79.0, 67.0), ("16", 105.0, 89.0), ("25", 133.0, 113.0), ("35", 166.0, 141.0),
        ("50", 201.0, 171.0), ("70", 259.0, 219.0), ("95", 315.0, 266.0), ("120", 367.0, 309.0),
        ("150", 424.0, 357.0), ("185", 486.0, 408.0), ("240", 575.0, 482.0), ("300", 664.0, 557.0),
        ("400", 777.0, 664.0), ("500", 903.0, 791.0),
    ];
    for (size, c2, c3) in pvc_f.iter() {
        table.insert((*size, "PVC", "F"), (*c2, *c3));
    }

    // ============================================================
    // XLPE 절연 (90°C) - IEC 60364-5-52 Table B.52.5
    // 주변온도 30°C (공기) / 20°C (지중) 기준
    // ============================================================

    // A1: 단열벽 속 전선관 (단심) - Table B.52.5 Column 2/3
    let xlpe_a1 = [
        ("1.5", 19.5, 17.0), ("2.5", 26.0, 23.0), ("4", 35.0, 31.0), ("6", 45.0, 40.0),
        ("10", 61.0, 54.0), ("16", 81.0, 73.0), ("25", 106.0, 95.0), ("35", 131.0, 117.0),
        ("50", 158.0, 141.0), ("70", 200.0, 179.0), ("95", 241.0, 216.0), ("120", 278.0, 249.0),
        ("150", 318.0, 285.0), ("185", 362.0, 324.0), ("240", 424.0, 380.0), ("300", 486.0, 435.0),
        ("400", 561.0, 503.0), ("500", 645.0, 578.0),
    ];
    for (size, c2, c3) in xlpe_a1.iter() {
        table.insert((*size, "XLPE", "A1"), (*c2, *c3));
    }
    
    // A2: 단열벽 속 전선관 (다심) - Table B.52.5 Column 4/5
    let xlpe_a2 = [
        ("1.5", 18.5, 16.5), ("2.5", 25.0, 22.0), ("4", 33.0, 30.0), ("6", 42.0, 38.0),
        ("10", 57.0, 51.0), ("16", 76.0, 68.0), ("25", 99.0, 89.0), ("35", 121.0, 109.0),
        ("50", 145.0, 130.0), ("70", 183.0, 164.0), ("95", 220.0, 197.0), ("120", 253.0, 227.0),
        ("150", 290.0, 259.0), ("185", 329.0, 295.0), ("240", 386.0, 346.0), ("300", 442.0, 396.0),
        ("400", 511.0, 458.0), ("500", 587.0, 526.0),
    ];
    for (size, c2, c3) in xlpe_a2.iter() {
        table.insert((*size, "XLPE", "A2"), (*c2, *c3));
    }

    // B1: 벽면 고정 전선관 (단심) - Table B.52.5 Column 6/7
    let xlpe_b1 = [
        ("1.5", 23.0, 20.0), ("2.5", 31.0, 28.0), ("4", 42.0, 37.0), ("6", 54.0, 48.0),
        ("10", 75.0, 66.0), ("16", 100.0, 88.0), ("25", 133.0, 117.0), ("35", 164.0, 144.0),
        ("50", 198.0, 175.0), ("70", 253.0, 222.0), ("95", 306.0, 269.0), ("120", 354.0, 312.0),
        ("150", 407.0, 358.0), ("185", 464.0, 408.0), ("240", 546.0, 481.0), ("300", 628.0, 553.0),
        ("400", 732.0, 644.0), ("500", 846.0, 745.0),
    ];
    for (size, c2, c3) in xlpe_b1.iter() {
        table.insert((*size, "XLPE", "B1"), (*c2, *c3));
    }

    // B2: 벽면 고정 전선관 (다심) - Table B.52.5 Column 8/9
    let xlpe_b2 = [
        ("1.5", 22.0, 19.5), ("2.5", 30.0, 27.0), ("4", 40.0, 35.0), ("6", 51.0, 45.0),
        ("10", 69.0, 62.0), ("16", 91.0, 82.0), ("25", 119.0, 107.0), ("35", 146.0, 131.0),
        ("50", 175.0, 158.0), ("70", 221.0, 200.0), ("95", 265.0, 240.0), ("120", 305.0, 276.0),
        ("150", 349.0, 316.0), ("185", 395.0, 358.0), ("240", 462.0, 419.0), ("300", 528.0, 479.0),
        ("400", 609.0, 553.0), ("500", 698.0, 635.0),
    ];
    for (size, c2, c3) in xlpe_b2.iter() {
        table.insert((*size, "XLPE", "B2"), (*c2, *c3));
    }

    // C: 벽면 직접 고정 - Table B.52.5 Column 10/11
    let xlpe_c = [
        ("1.5", 24.0, 22.0), ("2.5", 33.0, 30.0), ("4", 45.0, 40.0), ("6", 58.0, 52.0),
        ("10", 80.0, 71.0), ("16", 107.0, 96.0), ("25", 138.0, 119.0), ("35", 171.0, 147.0),
        ("50", 209.0, 179.0), ("70", 269.0, 229.0), ("95", 328.0, 278.0), ("120", 382.0, 322.0),
        ("150", 441.0, 371.0), ("185", 506.0, 424.0), ("240", 599.0, 500.0), ("300", 693.0, 576.0),
        ("400", 812.0, 673.0), ("500", 942.0, 778.0),
    ];
    for (size, c2, c3) in xlpe_c.iter() {
        table.insert((*size, "XLPE", "C"), (*c2, *c3));
    }

    // D1: 지중 덕트 - Table B.52.5 Column 12/13
    let xlpe_d1 = [
        ("1.5", 28.0, 22.0), ("2.5", 36.0, 29.0), ("4", 46.0, 37.0), ("6", 57.0, 46.0),
        ("10", 75.0, 60.0), ("16", 97.0, 77.0), ("25", 123.0, 99.0), ("35", 149.0, 119.0),
        ("50", 176.0, 140.0), ("70", 218.0, 173.0), ("95", 259.0, 204.0), ("120", 295.0, 233.0),
        ("150", 334.0, 263.0), ("185", 376.0, 295.0), ("240", 434.0, 340.0), ("300", 492.0, 384.0),
        ("400", 565.0, 441.0), ("500", 641.0, 499.0),
    ];
    for (size, c2, c3) in xlpe_d1.iter() {
        table.insert((*size, "XLPE", "D1"), (*c2, *c3));
    }

    // D2: 지중 직매 - Table B.52.5 Column 14/15
    let xlpe_d2 = [
        ("1.5", 31.0, 24.0), ("2.5", 41.0, 31.0), ("4", 52.0, 40.0), ("6", 65.0, 50.0),
        ("10", 85.0, 66.0), ("16", 110.0, 85.0), ("25", 141.0, 109.0), ("35", 170.0, 132.0),
        ("50", 202.0, 156.0), ("70", 251.0, 193.0), ("95", 300.0, 229.0), ("120", 343.0, 261.0),
        ("150", 390.0, 296.0), ("185", 440.0, 333.0), ("240", 510.0, 385.0), ("300", 578.0, 436.0),
        ("400", 664.0, 500.0), ("500", 753.0, 566.0),
    ];
    for (size, c2, c3) in xlpe_d2.iter() {
        table.insert((*size, "XLPE", "D2"), (*c2, *c3));
    }

    // E: 케이블 트레이 다심 (자유 공기 중) - Table B.52.12
    let xlpe_e = [
        ("1.5", 26.0, 23.0), ("2.5", 36.0, 32.0), ("4", 49.0, 42.0), ("6", 63.0, 54.0),
        ("10", 86.0, 75.0), ("16", 115.0, 100.0), ("25", 149.0, 127.0), ("35", 185.0, 158.0),
        ("50", 225.0, 192.0), ("70", 289.0, 246.0), ("95", 352.0, 298.0), ("120", 410.0, 346.0),
        ("150", 473.0, 399.0), ("185", 542.0, 456.0), ("240", 641.0, 538.0), ("300", 741.0, 621.0),
        ("400", 868.0, 742.0), ("500", 1008.0, 887.0),
    ];
    for (size, c2, c3) in xlpe_e.iter() {
        table.insert((*size, "XLPE", "E"), (*c2, *c3));
    }
    
    // F: 케이블 트레이 단심 (접촉 배치) - Table B.52.13 (단심 Touching/Trefoil)
    let xlpe_f = [
        ("1.5", 29.0, 25.0), ("2.5", 40.0, 35.0), ("4", 55.0, 47.0), ("6", 71.0, 60.0),
        ("10", 96.0, 83.0), ("16", 128.0, 111.0), ("25", 166.0, 141.0), ("35", 206.0, 176.0),
        ("50", 251.0, 214.0), ("70", 323.0, 274.0), ("95", 393.0, 332.0), ("120", 458.0, 386.0),
        ("150", 529.0, 445.0), ("185", 606.0, 509.0), ("240", 717.0, 601.0), ("300", 829.0, 694.0),
        ("400", 971.0, 828.0), ("500", 1127.0, 990.0),
    ];
    for (size, c2, c3) in xlpe_f.iter() {
        table.insert((*size, "XLPE", "F"), (*c2, *c3));
    }

    table
}

/// 전선 종류별 외경 데이터 (mm) - KEC 기준 제조사 규격
/// TFR-CV: 0.6/1kV 가교폴리에틸렌 절연 난연 PVC 시스 케이블
fn get_cable_outer_diameter(cable_type: &str, size: &str, cores: &str) -> Option<f64> {
    // TFR-CV 케이블 외경 (dcord.com 기준)
    let tfr_cv_1c: HashMap<&str, f64> = [
        ("1.5", 6.3), ("2.5", 6.7), ("4", 7.2), ("6", 7.8),
        ("10", 9.4), ("16", 10.0), ("25", 12.0), ("35", 13.0),
        ("50", 14.5), ("70", 16.0), ("95", 18.5), ("120", 20.0),
        ("150", 22.0), ("185", 24.0), ("240", 27.0), ("300", 30.0),
        ("400", 34.0), ("500", 37.0),
    ].iter().cloned().collect();

    let tfr_cv_2c: HashMap<&str, f64> = [
        ("1.5", 11.0), ("2.5", 12.0), ("4", 13.0), ("6", 14.0),
        ("10", 18.0), ("16", 21.0), ("25", 25.0), ("35", 29.0),
        ("50", 34.0), ("70", 39.0), ("95", 44.0), ("120", 50.0),
        ("150", 55.0), ("185", 61.0), ("240", 67.0), ("300", 75.0),
    ].iter().cloned().collect();

    // 3C 외경은 2C의 약 1.15배
    // 4C 외경은 2C의 약 1.25배

    // HFIX 전선 외경 (nexans, daeshincable 기준)
    let hfix_1c: HashMap<&str, f64> = [
        ("1.5", 3.3), ("2.5", 4.0), ("4", 4.6), ("6", 5.2),
        ("10", 6.5), ("16", 8.0), ("25", 10.1), ("35", 11.3),
        ("50", 13.2), ("70", 15.5), ("95", 18.0), ("120", 20.0),
        ("150", 22.5), ("185", 25.0), ("240", 28.5), ("300", 32.0),
    ].iter().cloned().collect();

    // CV 케이블 (TFR-CV와 유사하나 약간 작음)
    let cv_1c: HashMap<&str, f64> = [
        ("1.5", 6.0), ("2.5", 6.4), ("4", 6.9), ("6", 7.5),
        ("10", 9.0), ("16", 9.6), ("25", 11.5), ("35", 12.5),
        ("50", 14.0), ("70", 15.5), ("95", 18.0), ("120", 19.5),
        ("150", 21.5), ("185", 23.5), ("240", 26.5), ("300", 29.5),
        ("400", 33.0), ("500", 36.0),
    ].iter().cloned().collect();

    let cv_2c: HashMap<&str, f64> = [
        ("1.5", 10.5), ("2.5", 11.5), ("4", 12.5), ("6", 13.5),
        ("10", 17.0), ("16", 20.0), ("25", 24.0), ("35", 28.0),
        ("50", 33.0), ("70", 38.0), ("95", 43.0), ("120", 49.0),
        ("150", 54.0), ("185", 60.0), ("240", 66.0), ("300", 74.0),
    ].iter().cloned().collect();

    // FR-CV (내화 케이블) - TFR-CV보다 약간 큼
    let fr_cv_1c: HashMap<&str, f64> = [
        ("1.5", 6.8), ("2.5", 7.2), ("4", 7.7), ("6", 8.3),
        ("10", 10.0), ("16", 10.6), ("25", 12.6), ("35", 13.6),
        ("50", 15.1), ("70", 16.6), ("95", 19.1), ("120", 20.6),
        ("150", 22.6), ("185", 24.6), ("240", 27.6), ("300", 30.6),
        ("400", 34.6), ("500", 37.6),
    ].iter().cloned().collect();

    // TFR-8 (내열 케이블)
    let tfr_8_1c: HashMap<&str, f64> = [
        ("1.5", 6.5), ("2.5", 6.9), ("4", 7.4), ("6", 8.0),
        ("10", 9.6), ("16", 10.2), ("25", 12.2), ("35", 13.2),
        ("50", 14.7), ("70", 16.2), ("95", 18.7), ("120", 20.2),
        ("150", 22.2), ("185", 24.2), ("240", 27.2), ("300", 30.2),
    ].iter().cloned().collect();

    match (cable_type, cores) {
        ("HFIX", "1C") => hfix_1c.get(size).copied(),
        ("HFIX", _) => None, // HFIX는 단심만 존재
        
        ("TFR-CV", "1C") => tfr_cv_1c.get(size).copied(),
        ("TFR-CV", "2C") => tfr_cv_2c.get(size).copied(),
        ("TFR-CV", "3C") => tfr_cv_2c.get(size).map(|d| d * 1.15),
        ("TFR-CV", "4C") => tfr_cv_2c.get(size).map(|d| d * 1.25),
        
        ("CV", "1C") => cv_1c.get(size).copied(),
        ("CV", "2C") => cv_2c.get(size).copied(),
        ("CV", "3C") => cv_2c.get(size).map(|d| d * 1.15),
        ("CV", "4C") => cv_2c.get(size).map(|d| d * 1.25),

        ("FR-CV", "1C") => fr_cv_1c.get(size).copied(),
        ("FR-CV", "2C") => fr_cv_1c.get(size).map(|d| d * 1.65),
        ("FR-CV", "3C") => fr_cv_1c.get(size).map(|d| d * 1.9),
        ("FR-CV", "4C") => fr_cv_1c.get(size).map(|d| d * 2.1),

        ("TFR-8", "1C") => tfr_8_1c.get(size).copied(),
        ("TFR-8", "2C") => tfr_8_1c.get(size).map(|d| d * 1.65),
        ("TFR-8", "3C") => tfr_8_1c.get(size).map(|d| d * 1.9),
        ("TFR-8", "4C") => tfr_8_1c.get(size).map(|d| d * 2.1),

        _ => None,
    }
}

/// 전선 단면적 계산 (외경 기준, 원형)
fn calculate_cable_area(outer_diameter: f64) -> f64 {
    std::f64::consts::PI * (outer_diameter / 2.0).powi(2)
}

/// 전선관 내경 데이터 (mm) - 후강전선관 기준
fn get_conduit_data() -> Vec<(&'static str, f64)> {
    vec![
        ("C16 (16mm)", 15.8),
        ("C22 (22mm)", 21.0),
        ("C28 (28mm)", 26.6),
        ("C36 (36mm)", 35.0),
        ("C42 (42mm)", 41.0),
        ("C54 (54mm)", 53.0),
        ("C70 (70mm)", 69.0),
        ("C82 (82mm)", 80.0),
        ("C92 (92mm)", 89.0),
        ("C104 (104mm)", 101.0),
    ]
}

/// KEC 232.2 기준 추천 전선관 크기 계산 (33% 점유율)
fn recommend_conduit(total_area: f64) -> (String, f64) {
    let conduits = get_conduit_data();
    let max_fill_rate = 0.33; // KEC 232.2: 1/3 (33%) 이하

    for (name, inner_diameter) in conduits {
        let conduit_area = std::f64::consts::PI * (inner_diameter / 2.0).powi(2);
        let available_area = conduit_area * max_fill_rate;
        
        if available_area >= total_area {
            let actual_fill = (total_area / conduit_area) * 100.0;
            return (name.to_string(), actual_fill);
        }
    }

    ("C104 이상 검토 필요".to_string(), 100.0)
}

/// 전선 종류에 따른 절연체 반환
fn get_insulation_type(cable_type: &str) -> &'static str {
    match cable_type {
        "HFIX" => "XLPE",     // 저독성 가교 폴리올레핀 (90°C)
        "CV" | "TFR-CV" | "FR-CV" | "TFR-8" => "XLPE",  // 가교 폴리에틸렌 (90°C)
        _ => "PVC",          // 비닐 (70°C)
    }
}

/// 집합 보정 계수 (KEC Table B.52.17)
fn get_grouping_factor(num_circuits: u32) -> f64 {
    match num_circuits {
        0 | 1 => 1.00,
        2 => 0.80,
        3 => 0.70,
        4 => 0.65,
        5 => 0.60,
        6 => 0.57,
        7 => 0.54,
        8 => 0.52,
        9 => 0.50,
        10..=12 => 0.45,
        13..=16 => 0.41,
        17..=20 => 0.38,
        _ => 0.38, // 20회로 초과 시 0.38 적용 (보수적 접근)
    }
}

/// 공사방법 설명
fn get_install_method_description(method: &str) -> String {
    match method {
        "A1" => "단열벽 속 전선관 (절연전선/단심 케이블)".to_string(),
        "A2" => "단열벽 속 전선관 (다심 케이블)".to_string(),
        "B1" => "벽면 고정 전선관 (절연전선/단심 케이블)".to_string(),
        "B2" => "벽면 고정 전선관 (다심 케이블)".to_string(),
        "C" => "벽면/천정 직접 고정 (공기 중)".to_string(),
        "D1" => "지중 매설 덕트".to_string(),
        "D2" => "지중 매설 직매".to_string(),
        "E" => "케이블 트레이 (천공형, 단심)".to_string(),
        "F" => "케이블 트레이 (천공형, 다심)".to_string(),
        _ => "기타".to_string(),
    }
}

/// 메인 계산 함수 (Tauri 커맨드)
#[tauri::command]
fn calculate(data: CableData) -> Result<CalculationResult, String> {
    // 외경 계산
    let outer_diameter = get_cable_outer_diameter(&data.cable_type, &data.size, &data.cores)
        .ok_or("지원하지 않는 전선 규격입니다.")?;
    
    // 단위 케이블 단면적 (외경 기준)
    let single_cable_area = calculate_cable_area(outer_diameter);
    
    // 총 단면적 (수량 적용) - 전선관 채움률 계산용
    let mut total_area = single_cable_area * data.quantity as f64;
    
    // 도체 단면적 계산
    let conductor_area: f64 = data.size.parse::<f64>().unwrap_or(0.0) * data.quantity as f64;

    // 접지선 단면적 추가 (HFIX)
    if data.ground_wire == "HFIX" {
        // 접지선 규격 (주 전선의 약 50%)
        let ground_size = match data.size.as_str() {
            "1.5" | "2.5" => "1.5",
            "4" | "6" => "2.5",
            "10" | "16" => "6",
            "25" | "35" => "16",
            "50" | "70" => "25",
            "95" | "120" => "35",
            "150" | "185" => "70",
            _ => "95",
        };
        if let Some(ground_od) = get_cable_outer_diameter("HFIX", ground_size, "1C") {
            total_area += calculate_cable_area(ground_od);
        }
    }

    // 절연체 종류 결정
    let insulation = get_insulation_type(&data.cable_type);
    
    // 공사방법 결정
    let install_method = if data.install_method.is_empty() {
        match data.cores.as_str() {
            "1C" => "B1",
            _ => "B2",
        }
    } else {
        &data.install_method
    };

    // 허용전류 테이블에서 값 조회 (2부하/3부하 분리)
    let current_table = get_allowable_current_table();
    let current_values = current_table
        .get(&(data.size.as_str(), insulation, install_method))
        .ok_or("허용전류 데이터를 찾을 수 없습니다.")?;

    // 시스템 및 심선 수에 따른 부하 도체 수 판단
    // 1Φ (단상) -> 2 Loaded (2가닥 부하)
    // 3Φ (3상) -> 3 Loaded (3가닥 부하)
    let (base_current, loaded_label) = match data.system.as_str() {
        "1Φ" => (current_values.0, "2부하(단상)"),
        "3Φ" => (current_values.1, "3부하(3상)"),
        _ => (current_values.0, "2부하(기본)"),
    };

    // 집합 보정 계수 (Grouping Factor) 계산
    // 1C(단심) 케이블인 경우, 회로 수 계산:
    // 1Φ: 2가닥 = 1회로
    // 3Φ: 3가닥 = 1회로
    let num_circuits = if data.cores == "1C" {
        let cables_per_circuit = if data.system == "1Φ" { 2 } else { 3 };
        // 올림 계산 (남는 케이블이 있으면 회로로 간주)
        (data.quantity + cables_per_circuit - 1) / cables_per_circuit
    } else {
        // 다심 케이블은 수량 자체가 회로 수
        data.quantity
    };
    
    let grouping_factor = get_grouping_factor(num_circuits);

    // 심선 수 감소계수 (기존 코드는 이걸로 3상 변환을 시도했으나, 이제 표준 테이블 사용)
    // 그러나 "1C"가 아닌 "2C/3C/4C" 케이블 자체의 열적 특성은 이미 테이블에 반영됨 (2/3 loaded)
    // 단, 4C 케이블의 경우 KEC에서 3부하 도체로 간주하므로 추가 감소 없음 (중성선 부하 제외 가정)
    // 따라서 별도의 심선 수 감소계수는 삭제하고, Grouping Factor와 Loaded Table로 대체함.

    // 최종 허용전류 계산
    // 허용전류 = 기본값 * 집합보정계수 * (온도보정계수 1.0 가정)
    let allowable_current = base_current * grouping_factor;

    // 추천 전선관 계산
    let (recommended_conduit, fill_rate) = recommend_conduit(total_area);

    // 공사방법 설명
    let install_method_desc = format!(
        "{} / {} / 집합계수: {:.2} ({}회로)",
        get_install_method_description(install_method),
        loaded_label,
        grouping_factor,
        num_circuits
    );

    Ok(CalculationResult {
        total_area: (total_area * 100.0).round() / 100.0,
        conductor_area: (conductor_area * 100.0).round() / 100.0,
        allowable_current: (allowable_current * 10.0).round() / 10.0,
        recommended_conduit,
        fill_rate: (fill_rate * 10.0).round() / 10.0,
        install_method_desc,
    })
}

/// 전선 종류 목록 반환 (KEC 기준)
#[tauri::command]
fn get_cable_types() -> Vec<CableTypeInfo> {
    vec![
        CableTypeInfo {
            code: "HFIX".to_string(),
            name: "HFIX (저독성 난연 전선)".to_string(),
            description: "KS C 3341, 저독성 난연 폴리올레핀 절연".to_string(),
            max_temp: 90,
            insulation: "XLPE".to_string(),
        },
        CableTypeInfo {
            code: "TFR-CV".to_string(),
            name: "TFR-CV (난연 트레이용)".to_string(),
            description: "0.6/1kV 가교폴리에틸렌 절연 난연 PVC 시스".to_string(),
            max_temp: 90,
            insulation: "XLPE".to_string(),
        },
        CableTypeInfo {
            code: "CV".to_string(),
            name: "CV (일반 전력 케이블)".to_string(),
            description: "0.6/1kV 가교폴리에틸렌 절연 비닐 시스".to_string(),
            max_temp: 90,
            insulation: "XLPE".to_string(),
        },
        CableTypeInfo {
            code: "FR-CV".to_string(),
            name: "FR-CV (내화 케이블)".to_string(),
            description: "0.6/1kV 내화 가교폴리에틸렌 절연".to_string(),
            max_temp: 90,
            insulation: "XLPE".to_string(),
        },
        CableTypeInfo {
            code: "TFR-8".to_string(),
            name: "TFR-8 (내열 케이블)".to_string(),
            description: "0.6/1kV 내열 가교폴리에틸렌 절연".to_string(),
            max_temp: 90,
            insulation: "XLPE".to_string(),
        },
    ]
}

/// 전선 종류별 지원 옵션 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableTypeOptions {
    pub cores: Vec<(String, String)>,           // (코드, 표시명)
    pub sizes: Vec<String>,                      // 규격 목록
    pub install_methods: Vec<(String, String)>, // (코드, 표시명)
}

/// 전선 종류별 지원 옵션 반환 (필터링 데이터)
#[tauri::command]
fn get_cable_options(cable_type: String) -> CableTypeOptions {
    // 기본 규격 목록
    let sizes_standard = vec![
        "1.5", "2.5", "4", "6", "10", "16", "25", "35",
        "50", "70", "95", "120", "150", "185", "240", "300",
    ].into_iter().map(String::from).collect::<Vec<_>>();

    let sizes_extended = vec![
        "1.5", "2.5", "4", "6", "10", "16", "25", "35",
        "50", "70", "95", "120", "150", "185", "240", "300", "400", "500",
    ].into_iter().map(String::from).collect::<Vec<_>>();

    // 단심 전용 공사방법
    let methods_single = vec![
        ("A1".to_string(), "A1: 단열벽 속 전선관 (단심)".to_string()),
        ("B1".to_string(), "B1: 벽면 고정 전선관 (단심)".to_string()),
        ("C".to_string(), "C: 벽면/천정 직접 고정".to_string()),
        ("D1".to_string(), "D1: 지중 매설 덕트".to_string()),
        ("E".to_string(), "E: 케이블 트레이 (단심)".to_string()),
    ];

    // 전체 공사방법 (단심/다심 모두 지원)
    let methods_all = vec![
        ("A1".to_string(), "A1: 단열벽 속 전선관 (단심)".to_string()),
        ("A2".to_string(), "A2: 단열벽 속 전선관 (다심)".to_string()),
        ("B1".to_string(), "B1: 벽면 고정 전선관 (단심)".to_string()),
        ("B2".to_string(), "B2: 벽면 고정 전선관 (다심)".to_string()),
        ("C".to_string(), "C: 벽면/천정 직접 고정".to_string()),
        ("D1".to_string(), "D1: 지중 매설 덕트".to_string()),
        ("D2".to_string(), "D2: 지중 매설 직매".to_string()),
        ("E".to_string(), "E: 케이블 트레이 (단심)".to_string()),
        ("F".to_string(), "F: 케이블 트레이 (다심)".to_string()),
    ];

    match cable_type.as_str() {
        "HFIX" => CableTypeOptions {
            cores: vec![("1C".to_string(), "1C (단심)".to_string())],
            sizes: sizes_standard,
            install_methods: methods_single,
        },
        "TFR-CV" | "CV" => CableTypeOptions {
            cores: vec![
                ("1C".to_string(), "1C (단심)".to_string()),
                ("2C".to_string(), "2C (2심)".to_string()),
                ("3C".to_string(), "3C (3심)".to_string()),
                ("4C".to_string(), "4C (4심)".to_string()),
            ],
            sizes: sizes_extended,
            install_methods: methods_all,
        },
        "FR-CV" | "TFR-8" => CableTypeOptions {
            cores: vec![
                ("1C".to_string(), "1C (단심)".to_string()),
                ("2C".to_string(), "2C (2심)".to_string()),
                ("3C".to_string(), "3C (3심)".to_string()),
                ("4C".to_string(), "4C (4심)".to_string()),
            ],
            sizes: sizes_standard,
            install_methods: methods_all,
        },
        _ => CableTypeOptions {
            cores: vec![],
            sizes: vec![],
            install_methods: vec![],
        },
    }
}

/// 가닥수에 따른 공사방법 필터링
#[tauri::command]
fn get_install_methods_for_cores(cores: String) -> Vec<(String, String)> {
    match cores.as_str() {
        "1C" => vec![
            ("A1".to_string(), "A1: 단열벽 속 전선관 (단심)".to_string()),
            ("B1".to_string(), "B1: 벽면 고정 전선관 (단심)".to_string()),
            ("C".to_string(), "C: 벽면/천정 직접 고정".to_string()),
            ("D1".to_string(), "D1: 지중 매설 덕트".to_string()),
            ("E".to_string(), "E: 케이블 트레이 (단심)".to_string()),
        ],
        "2C" | "3C" | "4C" => vec![
            ("A2".to_string(), "A2: 단열벽 속 전선관 (다심)".to_string()),
            ("B2".to_string(), "B2: 벽면 고정 전선관 (다심)".to_string()),
            ("C".to_string(), "C: 벽면/천정 직접 고정".to_string()),
            ("D1".to_string(), "D1: 지중 매설 덕트".to_string()),
            ("D2".to_string(), "D2: 지중 매설 직매".to_string()),
            ("F".to_string(), "F: 케이블 트레이 (다심)".to_string()),
        ],
        _ => vec![],
    }
}

/// 전압 방식에 따른 적합한 심선 수 반환
/// 단상 (1Φ): 1C (단심 독립), 2C (단상 2선), 3C (단상 3선)
/// 3상 (3Φ): 3C (3상 3선), 4C (3상 4선)
#[tauri::command]
fn get_cores_for_system(system: String, available_cores: Vec<String>) -> Vec<(String, String)> {
    let allowed_cores: Vec<&str> = match system.as_str() {
        "1Φ" => vec!["1C", "2C", "3C"],  // 단상: 단심, 2선, 3선
        "3Φ" => vec!["1C", "3C", "4C"],  // 3상: 단심, 3선, 4선
        _ => vec!["1C", "2C", "3C", "4C"],
    };

    let core_names = [
        ("1C", "1C (단심)"),
        ("2C", "2C (단상 2선)"),
        ("3C", "3C (단상 3선 / 3상 3선)"),
        ("4C", "4C (3상 4선)"),
    ];

    core_names
        .iter()
        .filter(|(code, _)| allowed_cores.contains(code) && available_cores.contains(&code.to_string()))
        .map(|(code, name)| (code.to_string(), name.to_string()))
        .collect()
}

/// 전선 규격 목록 반환
#[tauri::command]
fn get_cable_sizes() -> Vec<String> {
    vec![
        "1.5", "2.5", "4", "6", "10", "16", "25", "35",
        "50", "70", "95", "120", "150", "185", "240", "300", "400", "500",
    ].into_iter().map(String::from).collect()
}

/// 가닥수 목록 반환
#[tauri::command]
fn get_core_options() -> Vec<(String, String)> {
    vec![
        ("1C".to_string(), "1C (단심)".to_string()),
        ("2C".to_string(), "2C (2심)".to_string()),
        ("3C".to_string(), "3C (3심)".to_string()),
        ("4C".to_string(), "4C (4심)".to_string()),
    ]
}

/// 공사방법 목록 반환 (KEC 기준)
#[tauri::command]
fn get_install_methods() -> Vec<(String, String)> {
    vec![
        ("A1".to_string(), "A1: 단열벽 속 전선관 (단심)".to_string()),
        ("A2".to_string(), "A2: 단열벽 속 전선관 (다심)".to_string()),
        ("B1".to_string(), "B1: 벽면 고정 전선관 (단심)".to_string()),
        ("B2".to_string(), "B2: 벽면 고정 전선관 (다심)".to_string()),
        ("C".to_string(), "C: 벽면/천정 직접 고정".to_string()),
        ("D1".to_string(), "D1: 지중 매설 덕트".to_string()),
        ("D2".to_string(), "D2: 지중 매설 직매".to_string()),
        ("E".to_string(), "E: 케이블 트레이 (단심)".to_string()),
        ("F".to_string(), "F: 케이블 트레이 (다심)".to_string()),
    ]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            calculate,
            get_cable_types,
            get_cable_options,
            get_cores_for_system,
            get_install_methods_for_cores,
            get_cable_sizes,
            get_core_options,
            get_install_methods
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
