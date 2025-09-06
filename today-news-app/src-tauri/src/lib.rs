// Tauri에서 자바스크립트(frontend) → Rust(backend)로 호출할 수 있는 명령(커맨드)을 정의
// #[tauri::command]는 매크로로, 이 함수를 JS에서 invoke("greet", { name: "값" }) 형태로 호출할 수 있게 해줌
#[tauri::command]
fn greet(name: &str) -> String {
    // format! 매크로로 문자열 생성 (name을 받아 "Hello, {name}!" 메시지 반환)
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 모바일 빌드 시, 엔트리 포인트(앱 실행 시작점)로 지정되는 어트리뷰트
// 데스크탑 빌드에서는 무시됨
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Tauri 앱 실행을 위한 빌더 패턴 시작
    tauri::Builder::default()
        // 플러그인 등록: 여기서는 `tauri-plugin-opener`
        // opener는 OS 기본 앱(브라우저, 파일 탐색기 등)으로 외부 리소스를 열 수 있게 해줌
        .plugin(tauri_plugin_opener::init())
        // JS에서 호출 가능한 Rust 함수들을 등록
        // `generate_handler!`는 greet 같은 #[tauri::command] 함수들을 매핑해줌
        .invoke_handler(tauri::generate_handler![greet])
        // 앱 실행에 필요한 Context 정보 로드
        // (tauri.conf.json에 정의된 앱 이름, 아이콘, 윈도우 설정 등)
        .run(tauri::generate_context!())
        // 앱 실행 중 에러가 나면 메시지를 띄우고 종료
        .expect("error while running tauri application");
}
