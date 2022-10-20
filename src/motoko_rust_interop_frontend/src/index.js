import {motoko_rust_interop_backend} from "../../declarations/motoko_rust_interop_backend";
import {createActor} from "../../declarations/rust_demo_backend";
import {Principal} from "@dfinity/principal";

let canisterId = Principal.fromText("qvhpv-4qaaa-aaaaa-aaagq-cai");

const initCanister = async () => {
  try {
    const principal = await motoko_rust_interop_backend.init();
    console.log('Initialized', principal.toText());

    canisterId = principal;
  } catch (err) {
    console.error(err);
  }
}

const deleteCanister = async () => {
  try {
    const principal = await motoko_rust_interop_backend.delete();
    console.log('Delete', principal.toText());
  } catch (err) {
    console.error(err);
  }
}

const balanceCanister = async () => {
  try {
    const balance = await motoko_rust_interop_backend.cyclesBalance();
    console.log('Balance', balance);
  } catch (err) {
    console.error(err);
  }
}

const greet = async () => {
  try {
    const actor = createActor(canisterId);
    const answer = await actor.greet("David");
    console.log(answer);
  } catch (err) {
    console.error(err);
  }
}

const init = () => {
  const btnInit = document.querySelector("button#init");
  btnInit.addEventListener("click", initCanister);

  const btnDelete = document.querySelector("button#delete");
  btnDelete.addEventListener("click", deleteCanister);

  const btnBalance = document.querySelector("button#balance");
  btnBalance.addEventListener("click", balanceCanister);

  const btnGreet = document.querySelector("button#greet");
  btnGreet.addEventListener("click", greet);
};

document.addEventListener("DOMContentLoaded", init);
