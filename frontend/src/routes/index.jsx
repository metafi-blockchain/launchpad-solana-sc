import React, { Suspense, lazy } from "react";

import { Route, Routes } from "react-router-dom";
import { Buffer } from "buffer";
import ScrollToTop from "../components/scrool-to-top";

const Error404 = lazy(() => import("../components/error-page/error-404"));
const Homepage = lazy(() => import("../pages/homepage"));

window.Buffer = Buffer;

const AppRoutes = () => {
  return (
    <ScrollToTop>
      <Suspense fallback={<></>}>
        <Routes>
          <Route path="*" element={<Error404 />} />
          <Route path="/" element={<Homepage />} />
        </Routes>
      </Suspense>
    </ScrollToTop>
  );
};

export default AppRoutes;
