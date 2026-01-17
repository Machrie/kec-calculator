// KEC 전선관/허용전류 산출 - JavaScript 애플리케이션

// Tauri 2.0 API
const invoke = window.__TAURI__.core.invoke;

// DOM 요소
const elements = {
    cableType: document.getElementById('cableType'),
    cableTypeDesc: document.getElementById('cableTypeDesc'),
    cores: document.getElementById('cores'),
    size: document.getElementById('size'),
    installMethod: document.getElementById('installMethod'),
    quantity: document.getElementById('quantity'),
    totalArea: document.getElementById('totalArea'),
    conductorArea: document.getElementById('conductorArea'),
    allowableCurrent: document.getElementById('allowableCurrent'),
    installMethodDesc: document.getElementById('installMethodDesc'),
    conduitSize: document.getElementById('conduitSize'),
    fillRate: document.getElementById('fillRate'),
    fillBar: document.getElementById('fillBar'),
};

// 전역 상태
let currentCableOptions = null;

// 초기화
async function init() {
    try {
        // 전선 종류 목록 로드
        const cableTypes = await invoke('get_cable_types');
        cableTypes.forEach(type => {
            const option = document.createElement('option');
            option.value = type.code;
            option.textContent = type.name;
            option.dataset.description = type.description;
            option.dataset.maxTemp = type.max_temp;
            elements.cableType.appendChild(option);
        });

        // 이벤트 리스너 등록
        setupEventListeners();

        console.log('KEC Calculator initialized');
    } catch (error) {
        console.error('Initialization error:', error);
    }
}

// 이벤트 리스너 설정
function setupEventListeners() {
    // Step 1: 전선 종류 변경
    elements.cableType.addEventListener('change', async () => {
        await onCableTypeChange();
    });

    // Step 2: 전압 방식 변경
    document.querySelectorAll('input[name="system"]').forEach(radio => {
        radio.addEventListener('change', async () => {
            await onSystemChange();
        });
    });

    // Step 3: 가닥수 변경
    elements.cores.addEventListener('change', async () => {
        await onCoresChange();
    });

    // Step 4-6: 기타 변경 시 계산
    elements.size.addEventListener('change', calculate);
    elements.installMethod.addEventListener('change', calculate);
    elements.quantity.addEventListener('input', calculate);

    // Step 7: 접지선 변경
    document.querySelectorAll('input[name="groundWire"]').forEach(radio => {
        radio.addEventListener('change', calculate);
    });
}

// Step 1: 전선 종류 변경 처리
async function onCableTypeChange() {
    const selectedType = elements.cableType.value;

    // 하위 셀렉트 초기화 및 비활성화
    resetSelect(elements.cores, '전압 방식을 확인하세요');
    resetSelect(elements.size, '전선 종류를 먼저 선택하세요');
    resetSelect(elements.installMethod, '가닥수를 먼저 선택하세요');
    elements.cores.disabled = true;
    elements.size.disabled = true;
    elements.installMethod.disabled = true;

    if (!selectedType) {
        elements.cableTypeDesc.textContent = '';
        currentCableOptions = null;
        return;
    }

    // 전선 종류 설명 표시
    const selectedOption = elements.cableType.selectedOptions[0];
    if (selectedOption) {
        elements.cableTypeDesc.textContent = `${selectedOption.dataset.description} (최고 ${selectedOption.dataset.maxTemp}°C)`;
    }

    try {
        // Rust 백엔드에서 해당 전선 종류의 옵션 가져오기
        currentCableOptions = await invoke('get_cable_options', { cableType: selectedType });

        // 규격 옵션 설정 (전선 종류에 따라)
        resetSelect(elements.size, '선택하세요');
        currentCableOptions.sizes.forEach(size => {
            const option = document.createElement('option');
            option.value = size;
            option.textContent = `${size} mm²`;
            elements.size.appendChild(option);
        });
        elements.size.disabled = false;

        // 전압 방식에 따른 가닥수 필터링
        await onSystemChange();
    } catch (error) {
        console.error('Error loading cable options:', error);
    }
}

// Step 2: 전압 방식 변경 처리
async function onSystemChange() {
    if (!currentCableOptions) return;

    const system = getRadioValue('system');
    const availableCores = currentCableOptions.cores.map(c => c[0]);

    try {
        // Rust 백엔드에서 전압 방식에 맞는 가닥수 가져오기
        const cores = await invoke('get_cores_for_system', {
            system: system,
            availableCores: availableCores
        });

        // 가닥수 옵션 갱신
        resetSelect(elements.cores, '선택하세요');
        cores.forEach(([code, name]) => {
            const option = document.createElement('option');
            option.value = code;
            option.textContent = name;
            elements.cores.appendChild(option);
        });
        elements.cores.disabled = false;

        // 가닥수가 하나뿐인 경우 자동 선택
        if (cores.length === 1) {
            elements.cores.value = cores[0][0];
            await onCoresChange();
        } else {
            // 공사방법 초기화
            resetSelect(elements.installMethod, '가닥수를 먼저 선택하세요');
            elements.installMethod.disabled = true;
        }
    } catch (error) {
        console.error('Error loading cores for system:', error);
    }

    calculate();
}

// Step 3: 가닥수 변경 처리
async function onCoresChange() {
    const cores = elements.cores.value;

    if (!cores) {
        resetSelect(elements.installMethod, '가닥수를 먼저 선택하세요');
        elements.installMethod.disabled = true;
        return;
    }

    try {
        // Rust 백엔드에서 가닥수에 맞는 공사방법 가져오기
        const methods = await invoke('get_install_methods_for_cores', { cores: cores });

        // 공사방법 옵션 갱신
        resetSelect(elements.installMethod, '선택하세요');
        methods.forEach(([code, name]) => {
            const option = document.createElement('option');
            option.value = code;
            option.textContent = name;
            elements.installMethod.appendChild(option);
        });
        elements.installMethod.disabled = false;

        // 기본값 선택 (B1 또는 B2)
        const defaultMethod = cores === '1C' ? 'B1' : 'B2';
        const validMethods = methods.map(m => m[0]);
        if (validMethods.includes(defaultMethod)) {
            elements.installMethod.value = defaultMethod;
        }
    } catch (error) {
        console.error('Error loading install methods:', error);
    }

    calculate();
}

// 셀렉트 초기화 헬퍼
function resetSelect(selectEl, placeholder) {
    selectEl.innerHTML = `<option value="">${placeholder}</option>`;
}

// 선택된 라디오 버튼 값 가져오기
function getRadioValue(name) {
    const selected = document.querySelector(`input[name="${name}"]:checked`);
    return selected ? selected.value : '';
}

// 숫자 포맷팅
function formatNumber(num, decimals = 2) {
    if (num === null || num === undefined || isNaN(num)) {
        return '-';
    }
    return num.toLocaleString('ko-KR', {
        minimumFractionDigits: decimals,
        maximumFractionDigits: decimals
    });
}

// 계산 실행
async function calculate() {
    const cableType = elements.cableType.value;
    const cores = elements.cores.value;
    const size = elements.size.value;
    const installMethod = elements.installMethod.value;
    const quantity = parseInt(elements.quantity.value) || 1;
    const system = getRadioValue('system');
    const groundWire = getRadioValue('groundWire');

    // 필수 값 확인
    if (!cableType || !cores || !size || !installMethod) {
        resetResults();
        return;
    }

    try {
        const data = {
            cable_type: cableType,
            cores: cores,
            size: size,
            quantity: quantity,
            system: system,
            ground_wire: groundWire,
            install_method: installMethod,
        };

        const result = await invoke('calculate', { data });
        updateResults(result);
    } catch (error) {
        console.error('Calculation error:', error);
        showError(error);
    }
}

// 결과 업데이트
function updateResults(result) {
    // 총 단면적
    elements.totalArea.textContent = formatNumber(result.total_area);
    elements.conductorArea.textContent = formatNumber(result.conductor_area);

    // 허용 전류
    if (result.allowable_current > 0) {
        elements.allowableCurrent.textContent = formatNumber(result.allowable_current, 1);
        elements.allowableCurrent.classList.remove('error');
    } else {
        elements.allowableCurrent.textContent = '-';
    }

    // 공사방법 설명
    elements.installMethodDesc.textContent = result.install_method_desc;

    // 추천 전선관
    elements.conduitSize.textContent = result.recommended_conduit;
    elements.fillRate.textContent = formatNumber(result.fill_rate, 1);

    // 점유율 바 애니메이션
    const fillPercent = Math.min(result.fill_rate, 100);
    elements.fillBar.style.width = `${fillPercent}%`;

    // 점유율에 따른 색상 변경
    if (fillPercent <= 33) {
        elements.fillBar.style.background = 'linear-gradient(90deg, #10b981, #34d399)';
        elements.fillBar.className = 'conduit-bar safe';
    } else if (fillPercent <= 50) {
        elements.fillBar.style.background = 'linear-gradient(90deg, #f59e0b, #fbbf24)';
        elements.fillBar.className = 'conduit-bar warning';
    } else {
        elements.fillBar.style.background = 'linear-gradient(90deg, #ef4444, #f87171)';
        elements.fillBar.className = 'conduit-bar danger';
    }
}

// 에러 표시
function showError(error) {
    elements.allowableCurrent.textContent = '-';
    elements.installMethodDesc.textContent = typeof error === 'string' ? error : '계산 오류';
}

// 결과 초기화
function resetResults() {
    elements.totalArea.textContent = '0';
    elements.conductorArea.textContent = '0';
    elements.allowableCurrent.textContent = '-';
    elements.installMethodDesc.textContent = '모든 항목을 선택하세요';
    elements.conduitSize.textContent = '-';
    elements.fillRate.textContent = '-';
    elements.fillBar.style.width = '0%';
}

// 앱 시작
document.addEventListener('DOMContentLoaded', init);
