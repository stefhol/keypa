import './index.css';
import * as jose from 'jose'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
  useNavigate
} from "react-router-dom";
import { Header } from './Components/Ui/Header';
import ErrorPage from './ErrorPage';
import "./index.css";
import { Home as Home } from './routes/home/Home';
import { KeycardBase } from './routes/keycard/KeycardBase';
import { ManageKeycard } from './routes/keycard/ManageKeycard';
import { ShowAllUsers } from './routes/users/ShowAllUsers';
import { Login } from './routes/login/Login';
import { Main } from './routes/Main';
import { ChangeRequest } from './routes/request/ChangeRequest';
import { ShowAllRequestFromUser } from './routes/request/ShowAllRequestFromUser';
import { ShowRequests } from './routes/request/ShowRequests';
import { RequestBase } from './routes/request/RequestBase';

import { SelfUser, UserByUserId } from './routes/user/User';
import { UserBase } from './routes/user/UserBase';
import { LoadingProvider } from './util/Provider/LoadingProvider';
import { StatsDemo } from './routes/stats/StatsDemo';
import { GlobalKeycardList } from './routes/keycard/GlobalKeycardList';
import { Logs } from './routes/logs/Logs';
import UserContext, { IUserContext } from './context/UserContext';
import { CreateKeycardRequest, CreateRoomRequest, CreateTempRequest, RequestPicker } from './routes/request/CreateRequest';
import { decodeToken } from './util/token';
import { UseKeycard } from './routes/use-keycard/UseKeycard';
import { Email } from './routes/email/Email';

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
        path: "/email",
        element: <Email />
      },
      {
        path: "/use-keycard",
        element: <UseKeycard />
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
        element: <ShowRequests />
      },
      {
        path: "request",
        element: <RequestBase />,
        children: [
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
        path: "users",
        element: <ShowAllUsers />
      },
      {
        path: "logs",
        element: <Logs />
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
        const token = decodeToken(params);
        setjwt({ ...token })
      }
    } else {
      if (!(window.location.pathname === "/" || window.location.pathname === "/login")) {
        window.location.pathname = "/"

      }
    }
  }
  React.useEffect(() => {
    callback()

    return () => {
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
