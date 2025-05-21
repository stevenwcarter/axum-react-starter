import Footer from 'components/Footer';
import { Outlet } from 'react-router-dom';

export const PageTemplate = () => (
  <div className="h-full flex flex-col text-white">
    <div className="flex flex-col p-4 md:p-10">
      <Outlet />
    </div>
    <Footer />
  </div>
);

export default PageTemplate;
