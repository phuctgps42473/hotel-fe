use leptos::{prelude::*, task::spawn_local};
use leptos_router::{hooks::use_query, params::Params};
use serde::Serialize;

use crate::{
    layouts::public::{footer::Footer, header::Header},
    libs::fetcher::fetch,
};

#[derive(Params, PartialEq, Debug, Clone)]
#[allow(non_snake_case)]
struct VNPayQuery {
    vnp_Amount: Option<u64>,
    vnp_BankCode: Option<String>,
    vnp_BankTranNo: Option<String>,
    vnp_CardType: Option<String>,
    vnp_OrderInfo: Option<String>,
    vnp_PayDate: Option<String>,
    vnp_ResponseCode: Option<String>,
    vnp_TmnCode: Option<String>,
    vnp_TransactionNo: Option<String>,
    vnp_TransactionStatus: Option<String>,
    vnp_TxnRef: Option<u64>,
    vnp_SecureHash: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
struct PaymentCallBackRequest {
  #[serde(rename = "bookingId")]
  booking_id: u64,
  #[serde(rename = "transactionNo")]
  transaction_no: String,
  amount: u64
}

impl From<VNPayQuery> for PaymentCallBackRequest {
  fn from(value: VNPayQuery) -> Self {
      Self {
        booking_id: value.vnp_TxnRef.unwrap(),
        transaction_no: value.vnp_TransactionNo.as_ref().unwrap().to_string(),
        amount: value.vnp_Amount.unwrap()
      }
  }
}

#[component]
pub fn PaymentCallBack() -> impl IntoView {
    let query = use_query::<VNPayQuery>();

    Effect::new(move || {
        let req_body: PaymentCallBackRequest = PaymentCallBackRequest::from(query.read().as_ref().unwrap().clone());
        spawn_local(async move {
            let _ = fetch::<PaymentCallBackRequest, ()>("payment/return", "POST", Some(req_body)).await;
        });
    });

    view! {
      <Header />
        <div class="font-sans min-h-screen flex flex-col items-center bg-white text-gray-800">
            // Main Content Area
            <main class="flex-grow flex flex-col items-center justify-center p-4 text-center max-w-2xl mx-auto">
                // Checkmark icons
                <div class="flex space-x-6 mb-8">
                    <div class="bg-teal-custom p-4 rounded-full text-white text-3xl md:text-4xl aspect-square flex items-center justify-center">
                        <i class="fas fa-check"></i>
                    </div>
                    <div class="bg-teal-custom p-4 rounded-full text-white text-3xl md:text-4xl aspect-square flex items-center justify-center">
                        <i class="fas fa-check"></i>
                    </div>
                    <div class="bg-teal-custom p-4 rounded-full text-white text-3xl md:text-4xl aspect-square flex items-center justify-center">
                        <i class="fas fa-check"></i>
                    </div>
                </div>

                // Success Message
                <h1 class="text-3xl md:text-4xl font-bold text-teal-custom uppercase">
                    "BẠN ĐÃ THANH TOÁN THÀNH CÔNG"
                </h1>

                // Illustration (Placeholder for the actual image)
                // You would replace this SVG with your actual image asset
                <div class="w-full max-w-sm">
                    <svg viewBox="0 0 400 300" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <rect x="0" y="0" width="400" height="300" fill="#FFFFFF"/>
                        <circle cx="200" cy="150" r="100" fill="#F5F5F5"/> // Outer circle
                        // Inner "browser window" illustration
                        <rect x="100" y="90" width="200" height="120" rx="10" fill="white" stroke="#E0E0E0" stroke-width="2"/>
                        <rect x="115" y="105" width="80" height="8" rx="2" fill="#E0E0E0"/>
                        <rect x="115" y="120" width="120" height="8" rx="2" fill="#E0E0E0"/>
                        <rect x="115" y="135" width="40" height="8" rx="2" fill="#E0E0E0"/>
                        // Smaller card on left
                        <rect x="65" y="140" width="70" height="80" rx="10" fill="white" stroke="#E0E0E0" stroke-width="2"/>
                        <circle cx="100" cy="155" r="12" fill="#3B7E97"/> // Blue user icon (info color from palette)
                        <rect x="75" y="175" width="50" height="6" rx="2" fill="#E0E0E0"/>
                        <rect x="75" y="185" width="30" height="6" rx="2" fill="#E0E0E0"/>
                        // Content blocks
                        <rect x="155" y="150" width="60" height="40" rx="5" fill="#E0E0E0"/>
                        <rect x="225" y="150" width="60" height="40" rx="5" fill="#E0E0E0"/>
                        <rect x="155" y="200" width="130" height="15" rx="5" fill="#E0E0E0"/>
                        // Green checkmark
                        <circle cx="260" cy="205" r="15" fill="#5B7B47"/> // Success color from palette
                        <path d="M255 205 L259 209 L265 201" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </div>


                // Instruction Message
                <p class="text-gray-600 text-lg mb-4">
                    "Vui lòng kiểm tra email và tin nhắn điện thoại của bạn."<br/>
                    "Chúng tôi đã gửi tất cả thông tin"
                </p>

                // Back to Home link
                <a href="/" class="text-teal-custom hover:text-teal-light text-lg font-semibold mt-4 block">
                    "Trở về trang chủ"
                </a>
            </main>
        </div>
        <Footer />
    }
}
