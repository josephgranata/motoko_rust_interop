import { motoko_rust_interop_backend } from "../../declarations/motoko_rust_interop_backend";

const initCanister = async () => {
  try {
    const principal = await motoko_rust_interop_backend.init();
    console.log('Initialized', principal.toText());
  } catch (err) {
    console.error(err);
  }
}

const init = () => {
  const btnInit = document.querySelector("button#init");
  btnInit.addEventListener("click", initCanister);
};

document.addEventListener("DOMContentLoaded", init);
