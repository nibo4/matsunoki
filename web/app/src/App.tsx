import { Component } from "solid-js";
import { Router, Routes, Route } from "solid-app-router";
import { SignUpPage } from "@matsunoki/ui";
import "@matsunoki/ui/lib/style.css";

export const App: Component = () => {
  return (
    <Router>
      <Routes>
        <Route path="/sign-up" component={SignUpPage} />
        <Route path="*" element={<p>NotFound</p>} />
      </Routes>
    </Router>
  );
};
