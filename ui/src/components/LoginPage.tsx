import { useState } from "react";
import Form from "../utils/form";

function LoginPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  return (
    <>
      <div className="border-2 flex justify-center flex-col h-80">
        <div>
          <h1 className="mb-5 font-extrabold text-3xl">Login</h1>
        </div>
        <div className="flex justify-center flex-col items-center">
          <Form
            fieldName="Email: "
            placeholder="email"
            value={email}
            onChange={(
              e: Event & {
                target: HTMLFormElement;
              },
            ) => setEmail(e.target.value)}
          />
          <Form
            fieldName="Password: "
            placeholder="password"
            value={password}
            onChange={(
              e: Event & {
                target: HTMLFormElement;
              },
            ) => setPassword(e.target.value)}
          />
          <div className="flex justify-between items-center">
            <input type="checkbox" />
            <h3 className="ml-2">Remember Me?</h3>
          </div>
          <div>
            <div className="flex justify-center items-center border-2 w-72 h-10 rounded-full">
              LogIn
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export default LoginPage;
