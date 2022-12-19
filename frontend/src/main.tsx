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
import { LeaderBase } from './routes/leader/LeaderBase';
import { ShowAllUsers } from './routes/leader/ShowAllUsers';
import { Login } from './routes/login/Login';
import { Main } from './routes/Main';
import { ChangeRequest } from './routes/request/ChangeRequest';
import { ShowAllRequestFromUser } from './routes/request/ShowAllRequestFromUser';
import { ShowPendingRequests } from './routes/request/ShowPendingRequests';
import { RequestBase } from './routes/request/RequestBase';
import { RequestPicker, TempRequest } from './routes/user/request/Request';
import { UserChange } from './routes/user/UseChange';
import { SelfUser, UserByUserId } from './routes/user/User';
import { UserBase } from './routes/user/UserBase';
import { LoadingProvider } from './util/Provider/LoadingProvider';
import { StatsDemo } from './routes/stats/StatsDemo';
import { GlobalKeycardList } from './routes/keycard/GlobalKeycardList';
import { Logs } from './routes/logs/Logs';
import { PropositionBase } from './routes/propositons/PropositionBase';
import { CreatePropostion } from './routes/propositons/CreatePropostion';
import { ConvertProposition } from './routes/propositons/ConvertProposition';
const router = createBrowserRouter([
  {
    path: "/",
    element: <Header />,
    children: [
      {
        path: "/stats",
        element: <StatsDemo />
      },
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
            element: <RequestPicker />
          },
          {
            path: "add-request/room",
            element: <CreatePropostion />
          },
          {
            path: "add-request/keycard",
            element: <KeycardRequest />
          },
          {
            path: "add-request/temp",
            element: <TempRequest />
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
            element: <GlobalKeycardList />
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
        ]
      },
      {
        path: "logs",
        element: <Logs />
      },
      {
        path: "propositions",
        element: <LeaderBase />,
        children: [

        ]
      },
      {
        path: "propositons",
        element: <RequestBase />,
        children: [
          {
            path: "",
            element: <PropositionBase />
          },
          {
            path: "demo",
            element: <ConvertProposition />
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
