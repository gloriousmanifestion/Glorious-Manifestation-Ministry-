# Microsoft Teams & Graph Integration

This guide covers enabling **Microsoft Teams Workflows** and **Teams classes** for the Pastoral School platform, so that:

- Classes can be created in Microsoft Teams (manually or via workflows).
- Students see their Teams classes on their dashboard, using Microsoft Graph API.

Your domain is already added to **Microsoft Exchange**; the steps below extend that to Teams and optional Power Automate workflows.

---

## Students with external emails (Gmail, Yahoo, etc.)

Students register in the Pastoral School app with **any email provider** (Gmail, Yahoo, Outlook.com, etc.). They do **not** need a Microsoft 365 or work/school account to use the app. To let them join your Teams classes:

- **Use guest access**: Your Microsoft 365 tenant can invite **external users (guests)** to a Team by email. When you add a student’s Gmail or Yahoo address to the team, Microsoft sends them an invite; they sign in with a one-time code or by creating/linking a Microsoft account (they keep their existing email). No need for you to give them an M365 license.
- **Enable guest access in Teams** (if not already on):
  - **Teams admin center** → **Users** → **Guest access**: set “Allow guest access in Teams” to **On**.
  - **Azure AD / Entra** → **External Identities** → **External collaboration settings**: allow “Guests can access” as needed (e.g. “Guest user access is restricted to their own directory objects” or your preferred level).
- **Adding students to a class/team**:
  - **Manually**: In Teams, open the team → **Add member** → type the student’s email (e.g. `student@gmail.com`) → add as **Guest**.
  - **Via Graph API**: Use the [invitation API](https://learn.microsoft.com/en-us/graph/api/invitation-post) or add the user as a guest to the group/team by email so they receive the invite.
- The **“Open in Teams”** link on the student dashboard works for these guests: when they open it, they’ll be prompted to sign in as a guest (one-time code or Microsoft account) and then see the class team.

So students from Gmail, Yahoo, or any other provider can participate in Teams classes as long as guest access is enabled and they are added to the team (manually or via automation).

---

## 1. Enable Workflows in Teams (tenant)

Workflows in Teams are Power Automate flows that run inside Teams. Ensure they are allowed for your tenant and for the right users.

### Admin steps (Microsoft 365 Admin Center)

1. **Teams admin**: Go to [Microsoft 365 Admin Center](https://admin.microsoft.com) → **Teams** → **Teams policies** (or **Meeting policies** / **Messaging policies** as relevant).
2. **Power Platform**: Go to **Settings** → **Services & add-ins** → **Power Platform** and ensure Power Automate (and if needed, Power Apps) are enabled for users who will create or run workflows.
3. **Allow Workflows in Teams**:
   - In **Teams admin center** → **Teams** → **Teams policies**: create or edit a policy and ensure **Allow user to create flows in Teams** (or equivalent) is **On** for educators/admins.
   - Alternatively, in **Power Platform admin center** → **Environments** → select your environment → **Settings** → **Product** → ensure **Teams integration** / **Workflows in Teams** is enabled.

Once enabled, users can create flows from the **Workflows** app in Teams or from the message overflow menu.

---

## 2. Creating classes in Teams

You can create Teams “classes” in two main ways: **Microsoft Graph API** (from your backend or scripts) or **Teams Workflows** (Power Automate in Teams).

### Option A: Microsoft Graph API – Education classes

The **Education API** creates class groups that can be used with Teams for Education:

- **Endpoint**: `POST https://graph.microsoft.com/v1.0/education/classes`
- **Permissions**: Application permission `EduRoster.ReadWrite.All` (least privileged for creating classes).
- **Note**: This creates the **education class** (M365 group with education properties). Teachers can then create a **Team** from that class in the Teams UI, or you can create a team from the group via Graph (see Option B for team creation).

Example payload:

```json
{
  "displayName": "Pastoral Leadership 2025",
  "mailNickname": "pastoral-leadership-2025",
  "description": "Pastoral School – Pastoral Leadership cohort",
  "classCode": "PL2025",
  "externalId": "pastoral-leadership",
  "externalName": "Pastoral Leadership",
  "externalSource": "manual"
}
```

- `externalId` can match your app’s course id (e.g. `pastoral-leadership`) so you can link **dashboard courses** to **Teams classes**.

### Option B: Microsoft Graph API – Create a Team (standard or from template)

To create a **Team** directly (with channels, tabs, etc.):

- **Endpoint**: `POST https://graph.microsoft.com/v1.0/teams`
- **Permissions**: `Team.Create`, and typically `Group.ReadWrite.All` (for the underlying M365 group).
- Returns `202 Accepted` and an operation URL; you poll until the team is provisioned, then add owners/members.

For **class-style** teams you can use the **EDU_Class** template when creating the team (template ID in Graph or via PowerShell). That gives you a class team layout (e.g. General, Assignments).

### Option C: Teams Workflows (Power Automate)

Use a **flow in Teams** to create a team or class when something happens (e.g. “When a new course is added” or “When a form is submitted”):

1. **Trigger**: e.g. “When a new item is created” (SharePoint list), “When an HTTP request is received”, or “When a flow is run from a message” in Teams.
2. **Action**: “Send an HTTP request to Microsoft Graph”:
   - Method: `POST`
   - URL: `https://graph.microsoft.com/v1.0/teams` or `https://graph.microsoft.com/v1.0/education/classes`
   - Headers: `Authorization: Bearer <token>`
   - Body: JSON as in Option A or B.

To get a token in the flow, use the **“Get token for Microsoft Graph”** action or an **Azure AD / Microsoft Entra** connector that uses an app registration with the right permissions (`Team.Create` or `EduRoster.ReadWrite.All`).

---

## 3. App registration and permissions (for backend or flows)

You already use an **Azure app registration** for Microsoft Graph (e.g. for `wangamail-rs` and transactional email). To create classes/teams and list them for students, add the following **application permissions** (if using app-only auth):

| Permission                | Use case                          |
|---------------------------|------------------------------------|
| `EduRoster.Read.All`      | List classes and members (read)    |
| `EduRoster.ReadWrite.All` | Create education classes           |
| `Team.Create`             | Create Teams (if not using Education API only) |
| `Group.ReadWrite.All`      | Often needed with Team.Create      |
| `User.Read.All`            | Resolve user by email (e.g. for roster) |

**Important**: Grant admin consent in Azure Portal for these permissions. Use the same `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, and `AZURE_CLIENT_SECRET` (or a dedicated app for Teams) in your backend or in Power Automate.

---

## 4. Showing Teams classes on the student dashboard

Two approaches:

### Approach 1: Link existing courses to Teams classes (recommended first step)

- In your **backend**, a column on `courses` stores the **Teams class id** or **education class id** (`teams_class_id`). Admins set it when creating or editing a course (e.g. paste the class/team id from Teams or Graph).
- **Student dashboard**: For the student’s **selected course**, if a `teams_class_id` exists, the app shows an **“Open in Teams”** link. No need to resolve the student in Azure AD; the link is the same for everyone in that course.
- **Adding students to the team**: Students must be added to the class/team in Microsoft 365 so the link works for them. Because students use **any email** (Gmail, Yahoo, etc.), add them as **guests** (see [Students with external emails](#students-with-external-emails-gmail-yahoo-etc) above). Once added, they open the link, sign in as guest if prompted, and see the class.

### Approach 2: List classes the student belongs to (by email)

- **Backend** endpoint (e.g. `GET /api/v1/auth/teams-classes`):
  - Use Graph to find the user by email (e.g. `GET https://graph.microsoft.com/v1.0/users?$filter=mail eq '<student_email>'`) or use **education** APIs if students are in the tenant.
  - Call `GET https://graph.microsoft.com/v1.0/education/me/classes` (delegated) or, with app permissions, list classes and filter by membership.
  - Return a list of class names + deep links.
- **Frontend**: On the dashboard, add a section **“Your Teams classes”** and render these classes with links to open in Teams.

**Caveat**: Approach 2 requires students to exist in your **Microsoft 365 tenant** (as guests or full users). If students only exist in your app’s database and not in Azure AD, use Approach 1 and add them to the class/team via Graph (e.g. add by email as guest) when they enroll.

---

## 5. Suggested implementation order

1. **Tenant**: Enable Workflows in Teams (section 1) and confirm your domain/Exchange setup.
2. **Azure**: Add Graph permissions to your app registration (section 3); grant admin consent.
3. **Backend**:
   - Add `teams_class_id` (or `education_class_id`) to courses (migration + admin API to set it).
   - Optional: New endpoint `GET /api/v1/auth/teams-classes` that calls Graph to list classes for the current user (if using Approach 2).
4. **Frontend**: In the dashboard Enrollment card (or a new card), show **“Open in Teams”** when the selected course has a linked Teams class; optionally add a **“Your Teams classes”** section if using Approach 2.
5. **Workflows**: Optionally add a Power Automate flow in Teams that creates a team/class (e.g. from a form or from an HTTP request from your backend when a new course is created).

---

## 6. Environment variables (backend)

Existing:

- `AZURE_TENANT_ID`
- `AZURE_CLIENT_ID`
- `AZURE_CLIENT_SECRET`

If you use a **different app** for Teams/Education, add e.g.:

- `TEAMS_APP_TENANT_ID`
- `TEAMS_APP_CLIENT_ID`
- `TEAMS_APP_CLIENT_SECRET`

---

## 7. References

- [Create educationClass – Microsoft Graph](https://learn.microsoft.com/en-us/graph/api/educationclass-post)
- [Create team – Microsoft Graph](https://learn.microsoft.com/en-us/graph/api/team-post)
- [Create flows in Microsoft Teams – Power Automate](https://learn.microsoft.com/en-us/power-automate/teams/teams-app-create)
- [Provisioning classes at scale (PowerShell + Graph)](https://support.microsoft.com/en-us/topic/provisioning-classes-at-scale-using-powershell-and-microsoft-graph-3428f18f-ff38-4c46-a696-3318e463cbec)
