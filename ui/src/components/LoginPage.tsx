import { useState } from "react";
import Form from "../utils/form";

function LoginPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  return (
    <>
      <div className="flex flex-col bg-transparent p-3">
        <div className="text-3xl font-extrabold p-3 text-white">Login</div>
        <div className="flex flex-col justify-around items-center h-32">
          <Form
            placeholder="Username"
            onChange={(e) => setEmail(e.target.value)}
            value={email}
          />
          <Form
            placeholder="Password"
            onChange={(e) => setPassword(e.target.value)}
            value={password}
          />
        </div>
        <div>
          <div className="flex justify-center">
            <div className="flex flex-row justify-between items-center w-72">
              <div className="flex flex-row justify-center items-center">
                <input type="checkbox" />
                <div className="ml-2 text-white">Remember me</div>
              </div>
              <div className="text-white">Forgot password</div>
            </div>
          </div>
        </div>
        <div className="flex justify-center items-center pb-3">
          <div className="text-2xl border-2 rounded-full mt-10 w-72 text-slate-500 bg-white">
            LogIn
          </div>
        </div>
        <div
          className="flex flex-row justify-between w-64 text-white cursor-pointer"
          onClick={() => console.log("clicked")}
        >
          <h1>Don't have an account?</h1>
          <h1 className="font-semibold">Register</h1>
        </div>
      </div>
    </>
  );
}

export default LoginPage;
