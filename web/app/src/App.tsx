import { Component } from "solid-js";
import { Router, Routes, Route, useNavigate } from "solid-app-router";
import { SignUpPage, CoreContext, SignUpDIContext } from "@matsunoki/ui";
import { initForProduction } from "@matsunoki/core";
import "@matsunoki/ui/lib/style.css";

const SignUp: Component = () => {
  const navigate = useNavigate();
  return (
    <SignUpDIContext.Provider
      value={{
        navigateToRoot: () => navigate("/"),
      }}
    >
      <SignUpPage />
    </SignUpDIContext.Provider>
  );
};
export const App: Component = () => {
  return (
    <CoreContext.Provider value={initForProduction()}>
      <Router>
        <Routes>
          <Route path="/sign-up" component={SignUp} />
          <Route path="*" element={<p>NotFound</p>} />
        </Routes>
      </Router>
    </CoreContext.Provider>
  );
};
