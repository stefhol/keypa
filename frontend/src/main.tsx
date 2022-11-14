import './index.css';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider
} from "react-router-dom";
import { Header } from './Components/Ui/Header';
import ErrorPage from './ErrorPage';
import "./index.css";
import { Dashboard } from './routes/dashboard/Dashboard';
import { KeycardsFromUser } from './routes/keycard/Keycard';
import { KeycardBase } from './routes/keycard/KeycardBase';
import { KeycardRequest } from './routes/keycard/KeycardRequest';
import { ManageKeycard } from './routes/keycard/ManageKeycard';
import { ChangeWorker } from './routes/leader/ChangeWorker';
import { LeaderBase } from './routes/leader/LeaderBase';
import { ShowAllUsers } from './routes/leader/ShowAllUsers';
import { Login } from './routes/login/Login';
import { Main } from './routes/Main';
import { ChangeRequest } from './routes/request/ChangeRequest';
import { ShowAllRequestFromUser } from './routes/request/ShowAllRequestFromUser';
import { ShowPendingRequests } from './routes/request/ShowPendingRequests';
import { RequestBase } from './routes/request/WorkerBase';
import { Request } from './routes/user/request/Request';
import { UserChange } from './routes/user/UseChange';
import { SelfUser, UserByUserId } from './routes/user/User';
import { UserBase } from './routes/user/UserBase';
import { LoadingProvider } from './util/Provider/LoadingProvider';
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
            element: <SelfUser />
          },
          {
            path: "account",
            element: <UserChange />
          }, {
            path: ":userId/account",
            element: <UserChange />
          },
          {
            path: ":userId/keycard",
            element: <KeycardsFromUser />
          },
          {
            path: ":userId/request",
            element: <ShowAllRequestFromUser />
          },
          {
            path: ":userId",
            element: <UserByUserId />
          }
        ]
      },
      {
        path: "request",
        element: <RequestBase />,
        children: [
          {
            path: "",
            element: <ShowPendingRequests />
          },
          {
            path: "add-request",
            element: <Request />
          },
          {
            path: "change-request/:requestId",
            element: <ChangeRequest />

          },
        ]
      },
      {
        path: "keycard",
        element: <KeycardBase />,
        children: [
          {
            path: "",
            element: <></>
          },
          {
            path: "add-request",
            element: <KeycardRequest />
          },
          {
            path: "change-request/:requestId",
            element: <ManageKeycard />
          },
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

    ]
  }

]);
const queryClient = new QueryClient()
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <LoadingProvider>
        <RouterProvider router={router} />
      </LoadingProvider>
    </QueryClientProvider>
  </React.StrictMode>
);