import { useState } from "react";
import Form from "../utils/form";
import "../index.css";

function LoginPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  return (
    <>
      <div className="border-2 bg-sky-400 flex flex-col">
        <div className="text-3xl font-extrabold p-3">Login</div>
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
                <div className="ml-2">Remember me</div>
              </div>
              <div>Forgot password</div>
            </div>
          </div>
        </div>
        <div className="flex justify-center items-center pb-3">
          <div className="text-2xl border-2 rounded-full mt-10 w-72 bg-orange-500">
            LogIn
          </div>
        </div>
      </div>
    </>
  );
}

export default LoginPage;
