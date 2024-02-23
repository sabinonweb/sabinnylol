import { createBrowserRouter, RouterProvider } from "react-router-dom";
import App from "../App";
import Loginpage from "../components/LoginPage.tsx";

export function Router() {
  const router = createBrowserRouter([
    {
      path: "/",
      element: <App />,
    },
    {
      path: "/login",
      element: <Loginpage />,
    },
  ]);

  return <RouterProvider route={router} />;
}
