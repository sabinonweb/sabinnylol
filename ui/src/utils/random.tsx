<div className="flex justify-center bg-sky-400 items-center flex-col border-2">
  <Form
    fieldName="Email: "
    placeholder="Username"
    value={email}
    onChange={(
      e: Event & {
        target: HTMLFormElement;
      },
    ) => setEmail(e.target.value)}
  />
  <Form
    fieldName="Password: "
    placeholder="Password"
    value={password}
    onChange={(
      e: Event & {
        target: HTMLFormElement;
      },
    ) => setPassword(e.target.value)}
  />
  <div className="flex justify-center items-center">
    <div className="flex flex-row bg-lime-500 justify-between w-72 mb-20">
      <div className="flex flex-row justify-center">
        <input type="checkbox" />
        <h3 className="ml-2 text-sm">Remember Me</h3>
      </div>
      <div>
        <h1 className="text-sm">Forgot password?</h1>
      </div>
    </div>
  </div>
</div>;
