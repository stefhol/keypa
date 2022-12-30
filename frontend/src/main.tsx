import './index.css';
import * as jose from 'jose'
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
import { Home as Home } from './routes/home/Home';
import { KeycardsFromUser } from './routes/keycard/Keycard';
import { KeycardBase } from './routes/keycard/KeycardBase';
import { ManageKeycard } from './routes/keycard/ManageKeycard';
import { LeaderBase } from './routes/leader/LeaderBase';
import { ShowAllUsers } from './routes/leader/ShowAllUsers';
import { Login } from './routes/login/Login';
import { Main } from './routes/Main';
import { ChangeRequest } from './routes/request/ChangeRequest';
import { ShowAllRequestFromUser } from './routes/request/ShowAllRequestFromUser';
import { ShowPendingRequests } from './routes/request/ShowPendingRequests';
import { RequestBase } from './routes/request/RequestBase';
import { UserChange } from './routes/user/UseChange';
import { SelfUser, UserByUserId } from './routes/user/User';
import { UserBase } from './routes/user/UserBase';
import { LoadingProvider } from './util/Provider/LoadingProvider';
import { StatsDemo } from './routes/stats/StatsDemo';
import { GlobalKeycardList } from './routes/keycard/GlobalKeycardList';
import { Logs } from './routes/logs/Logs';
import UserContext, { IUserContext } from './context/UserContext';
import { CreateKeycardRequest, CreateRoomRequest, CreateTempRequest, RequestPicker } from './routes/request/CreateRequest';
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
        path: "/home",
        element: <Home />,
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
            element: <CreateRoomRequest />
          },
          {
            path: "add-request/keycard",
            element: <CreateKeycardRequest />
          },
          {
            path: "add-request/temp",
            element: <CreateTempRequest />
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
    ]
  }

]);
const queryClient = new QueryClient()
const Default: React.FC<DefaultProps> = (props) => {
  const callback = () => {
    const params = new window.URLSearchParams(document.cookie)
    console.log(params);

    if (params.has("token")) {
      if (params.get("token")) {
        const token = jose.decodeJwt(params.get("token") as string) as {
          is_admin: boolean,
          is_leader: boolean,
          is_worker: boolean,
          sub: string,
        };
        setjwt({ ...token })
      }
    }
  }
  React.useEffect(() => {
    window.addEventListener("load", callback)
    return () => {
      window.removeEventListener("load", callback)
    }
  }, []);
  const [jwt, setjwt] = React.useState({} as IUserContext);
  return (<>
    <UserContext.Provider value={{ ...jwt, set: setjwt }}>
      {props.children}
    </UserContext.Provider>
  </>)
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Default>

    <QueryClientProvider client={queryClient}>
      <LoadingProvider>
        <RouterProvider router={router} />
      </LoadingProvider>
    </QueryClientProvider>
    </Default>

  </React.StrictMode>
);

interface DefaultProps {
  children: any;
}
