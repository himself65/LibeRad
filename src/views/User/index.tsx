import React, { Suspense } from 'react'

export const UserView: React.FC = () => {
  return (
    <div>
      <Suspense fallback='Loading...'>
        <span> User </span>
      </Suspense>
      <span>
        This is User Page
      </span>
    </div>
  )
}

export default UserView
