import App from './App'
import './index.css'

import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
  Route,
} from "react-router-dom";
import "./index.css";
import ErrorPage from './ErrorPage';
import { Login } from './routes/login/Login';
import { Main } from './routes/Main';
import { Dashboard } from './routes/dashboard/Dashboard';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Request } from './routes/user/request/Request';
import { Header } from './Components/Ui/Header';
import { LeaderBase } from './routes/leader/LeaderBase';
import { ChangeWorker } from './routes/leader/ChangeWorker';
import { UserBase } from './routes/user/UserBase';
import { ShowAllUsers } from './routes/leader/ShowAllUsers';
import { User } from './routes/user/User';
import { UserChange as UserChange } from './routes/user/UseChange';
import { WorkerBase } from './routes/worker/WorkerBase';
import { ShowPendingRequests } from './routes/worker/ShowPendingRequests';
import { ChangeRequest } from './routes/worker/ChangeRequest';
const router = createBrowserRouter([
  {
    path: "/",
    element: <Header />,
    children: [
      {
        path: "/",
        element: <Main />,
        errorElement: <ErrorPage />,
      },
      {
        path: "/login",
        element: <Login />,
      },
      {
        path: "/dashboard",
        element: <Dashboard />,
      },
      {
        path: "user",
        element: <UserBase />,
        children: [
          {
            path: "",
            element: <User />
          },
          {
            path: "request",
            element: <Request />
          },
          {
            path: "account",
            element: <UserChange />
          }
        ]
      },
      {
        path: "leader",
        element: <LeaderBase />,
        children: [
          {
            path: "",
            element: <ShowAllUsers />

          },
          {
            path: "change-worker/:userId",
            element: <ChangeWorker />
          }
        ]
      },
      {
        path: "worker",
        element: <WorkerBase />,
        children: [
          {
            path: "",
            element: <ShowPendingRequests />

          },
          {
            path: "change-request/:requestId",
            element: <ChangeRequest />

          },

        ]
      }
    ]
  }

]);
const queryClient = new QueryClient()
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>

      <RouterProvider router={router} />
    </QueryClientProvider>
  </React.StrictMode>
);
