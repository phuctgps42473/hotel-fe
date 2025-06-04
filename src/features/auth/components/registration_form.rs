use leptos::prelude::*;

#[component]
pub fn RegistrationForm() -> impl IntoView {
    view! {
      <div class="relative flex flex-col justify-center h-full overflow-scroll scrollbar-hide">
          <div class="w-full p-6 bg-base-300 m-auto rounded-md shadow-md ring-2 ring-gray-800/50 lg:max-w-xl">
              <h1 class="text-3xl font-semibold text-center">DaisyUI</h1>
              <form class="space-y-4">
                  <div>
                      <label class="label">
                          <span class="text-base label-text">Name</span>
                      </label>
                      <input type="text" placeholder="Name" class="w-full input input-bordered" />
                  </div>
                  <div>
                      <label class="label">
                          <span class="text-base label-text">Email</span>
                      </label>
                      <input type="text" placeholder="Email Address" class="w-full input input-bordered" />
                  </div>
                  <div>
                      <label class="label">
                          <span class="text-base label-text">Password</span>
                      </label>
                      <input type="password" placeholder="Enter Password" class="w-full input input-bordered" />
                  </div>
                  <div>
                      <label class="label">
                          <span class="text-base label-text">Confirm Password</span>
                      </label>
                      <input type="password" placeholder="Confirm Password" class="w-full input input-bordered" />
                  </div>
                  <div>
                      <button class="btn btn-block">Sign Up</button>
                  </div>
                  <span>Already have an account ?
                      <a href="#" class="text-blue-600 hover:text-blue-800 hover:underline">Login</a></span>
              </form>
              <div class="flex items-center w-full my-4">
                  <hr class="w-full" />
                  <p class="px-3 ">OR</p>
                  <hr class="w-full" />
              </div>
              <div class="my-6 space-y-2">
                  <button aria-label="Login with Google" type="button"
                      class="flex items-center justify-center w-full p-2 space-x-4 border rounded-md focus:ring-2 focus:ring-offset-1 focus:ring-gray-400">
                      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" class="w-5 h-5 fill-current">
                          <path
                              d="M16.318 13.714v5.484h9.078c-0.37 2.354-2.745 6.901-9.078 6.901-5.458 0-9.917-4.521-9.917-10.099s4.458-10.099 9.917-10.099c3.109 0 5.193 1.318 6.38 2.464l4.339-4.182c-2.786-2.599-6.396-4.182-10.719-4.182-8.844 0-16 7.151-16 16s7.156 16 16 16c9.234 0 15.365-6.49 15.365-15.635 0-1.052-0.115-1.854-0.255-2.651z">
                          </path>
                      </svg>
                      <p>Login with Google</p>
                  </button>
              </div>
          </div>
      </div>
    }
}
