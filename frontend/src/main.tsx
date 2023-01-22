import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider
} from "react-router-dom";
import { Header } from './Components/Ui/Header';
import ErrorPage from './ErrorPage';
import './index.css';
import { Home } from './routes/home/Home';
import { KeycardBase } from './routes/keycard/KeycardBase';
import { ManageKeycard } from './routes/keycard/ManageKeycard';
import { Login } from './routes/login/Login';
import { Main } from './routes/Main';
import { ChangeRequest } from './routes/request/ChangeRequest';
import { RequestBase } from './routes/request/RequestBase';
import { ShowAllRequestFromUser } from './routes/request/ShowAllRequestFromUser';
import { ShowRequests } from './routes/request/ShowRequests';
import { ShowAllUsers } from './routes/users/ShowAllUsers';

import i18next from 'i18next';
import UserContext, { IUserContext } from './context/UserContext';
import { GlobalKeycardList } from './routes/keycard/GlobalKeycardList';
import { Logs } from './routes/logs/Logs';
import { CreateKeycardRequest, CreateRoomRequest, CreateTempRequest, RequestPicker } from './routes/request/CreateRequest';
import { UseKeycard } from './routes/use-keycard/UseKeycard';
import { SelfUser, UserByUserId } from './routes/user/User';
import { UserBase } from './routes/user/UserBase';
import { LoadingProvider } from './util/Provider/LoadingProvider';
import { Rest } from './util/Rest';
import { decodeToken } from './util/token';

const router = createBrowserRouter([
  {
    path: "/",
    element: <Header />,
    children: [
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
if (!localStorage.getItem("language")) {
  localStorage.setItem("language", "en")
}
i18next.init({ defaultNS: '1', resources: {}, lng: localStorage.getItem("language") as string });
Rest.getRessourceBundle("en").then(res => {
  i18next.addResourceBundle("en", "1", res)

})
Rest.getRessourceBundle("de").then(res => {
  i18next.addResourceBundle("de", "1", res)
})

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
