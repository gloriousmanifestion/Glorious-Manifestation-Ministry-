/**
 * Course catalog: matches backend course ids and adds full details + schedule for course pages.
 * Backend courses: pastoral-leadership, prophetic-ministry, ministerial-training,
 * biblical-studies, worship-praise, evangelism-outreach
 */

import { SITE } from '$lib/config';

export interface CourseDetail {
	/** Backend course id (also used as URL slug) */
	id: string;
	name: string;
	description: string;
	/** Short one-liner for cards */
	shortDescription: string;
	icon: string;
	topics: string[];
	/** What to expect: schedule / outline for the course */
	schedule: { week?: string; title: string; description: string }[];
}

export const COURSE_CATALOG: CourseDetail[] = [
	{
		id: 'pastoral-leadership',
		name: 'Pastoral Leadership',
		shortDescription:
			'Develop biblical leadership skills for effective church ministry and shepherding.',
		description:
			'This course equips you with biblical leadership principles for church ministry. You will learn to shepherd, counsel, and lead with wisdom and integrity. Topics include church administration, pastoral care, leadership principles, and conflict resolution.',
		icon: '📖',
		topics: [
			'Church administration',
			'Pastoral care',
			'Leadership principles',
			'Conflict resolution'
		],
		schedule: [
			{
				week: 'Weeks 1–2',
				title: 'Foundations of pastoral leadership',
				description: 'Biblical models of leadership and the role of the pastor.'
			},
			{
				week: 'Weeks 3–4',
				title: 'Church administration',
				description: 'Stewarding resources, planning, and church governance.'
			},
			{
				week: 'Weeks 5–6',
				title: 'Pastoral care & counseling',
				description: 'Shepherding members through life challenges and spiritual growth.'
			},
			{
				week: 'Weeks 7–8',
				title: 'Conflict resolution',
				description: 'Biblical peacemaking and handling difficult situations.'
			}
		]
	},
	{
		id: 'prophetic-ministry',
		name: 'Prophetic Ministry',
		shortDescription:
			'Understanding and operating in the prophetic gift with wisdom and discernment.',
		description:
			'Learn to hear from God and operate in the prophetic gift with accuracy and wisdom. This course covers hearing God, prophecy and discernment, dreams and visions, and prophetic protocol in the local church.',
		icon: '✨',
		topics: ['Hearing God', 'Prophecy and discernment', 'Dreams and visions', 'Prophetic protocol'],
		schedule: [
			{
				week: 'Weeks 1–2',
				title: 'Hearing God',
				description: 'Ways God speaks and how to cultivate a listening heart.'
			},
			{
				week: 'Weeks 3–4',
				title: 'Prophecy and discernment',
				description: 'Delivering prophetic words with accuracy and testing spirits.'
			},
			{
				week: 'Weeks 5–6',
				title: 'Dreams and visions',
				description: 'Understanding and interpreting dreams and visions.'
			},
			{
				week: 'Weeks 7–8',
				title: 'Prophetic protocol',
				description: 'Order, accountability, and integrity in prophetic ministry.'
			}
		]
	},
	{
		id: 'ministerial-training',
		name: 'Ministerial Training',
		shortDescription:
			'Comprehensive training for ministers in preaching, teaching, and pastoral care.',
		description:
			'Comprehensive training for ministers in preaching, teaching, and pastoral care. Build confidence in public ministry through homiletics, teaching methods, pastoral counseling, and ministry ethics.',
		icon: '⛪',
		topics: ['Homiletics', 'Teaching methods', 'Pastoral counseling', 'Ministry ethics'],
		schedule: [
			{
				week: 'Weeks 1–2',
				title: 'Homiletics',
				description: 'Preparing and delivering biblical sermons.'
			},
			{
				week: 'Weeks 3–4',
				title: 'Teaching methods',
				description: 'Effective teaching and small group facilitation.'
			},
			{
				week: 'Weeks 5–6',
				title: 'Pastoral counseling',
				description: 'Basic counseling skills and referral.'
			},
			{
				week: 'Weeks 7–8',
				title: 'Ministry ethics',
				description: 'Integrity, boundaries, and accountability in ministry.'
			}
		]
	},
	{
		id: 'biblical-studies',
		name: 'Biblical Studies',
		shortDescription: 'In-depth study of Scripture for sound doctrine and spiritual growth.',
		description:
			'In-depth study of Scripture for sound doctrine and spiritual growth. Covers hermeneutics, systematic theology, Old and New Testament survey, and church history.',
		icon: '📜',
		topics: ['Hermeneutics', 'Systematic theology', 'Old & New Testament survey', 'Church history'],
		schedule: [
			{
				week: 'Weeks 1–2',
				title: 'Hermeneutics',
				description: 'How to interpret Scripture rightly.'
			},
			{
				week: 'Weeks 3–4',
				title: 'Systematic theology',
				description: 'Key doctrines of the faith.'
			},
			{
				week: 'Weeks 5–6',
				title: 'Old & New Testament survey',
				description: 'Overview of the Bible and its message.'
			},
			{
				week: 'Weeks 7–8',
				title: 'Church history',
				description: 'Key movements and lessons from church history.'
			}
		]
	},
	{
		id: 'worship-praise',
		name: 'Worship & Praise',
		shortDescription: 'Leading worship and understanding the ministry of praise in the church.',
		description:
			'Leading worship and understanding the ministry of praise in the church. Topics include worship leading, praise and worship, music ministry, and flow in the Spirit.',
		icon: '🎵',
		topics: ['Worship leading', 'Praise and worship', 'Music ministry', 'Flow in the Spirit'],
		schedule: [
			{
				week: 'Weeks 1–2',
				title: 'Heart of worship',
				description: 'Biblical foundations of worship and praise.'
			},
			{
				week: 'Weeks 3–4',
				title: 'Worship leading',
				description: 'Leading congregations in worship and flow.'
			},
			{
				week: 'Weeks 5–6',
				title: 'Music ministry',
				description: 'Practical skills and team building.'
			},
			{
				week: 'Weeks 7–8',
				title: 'Spirit-led worship',
				description: 'Sensitivity to the Spirit in corporate worship.'
			}
		]
	},
	{
		id: 'evangelism-outreach',
		name: 'Evangelism & Outreach',
		shortDescription: 'Equipping for effective soul-winning and community outreach.',
		description:
			'Equipping for effective soul-winning and community outreach. Share the Gospel with confidence and compassion. Covers personal evangelism, street ministry, community outreach, and discipleship.',
		icon: '🌍',
		topics: ['Personal evangelism', 'Street ministry', 'Community outreach', 'Discipleship'],
		schedule: [
			{
				week: 'Weeks 1–2',
				title: 'Personal evangelism',
				description: 'Sharing your faith naturally and clearly.'
			},
			{
				week: 'Weeks 3–4',
				title: 'Street ministry',
				description: 'Preaching and witnessing in public spaces.'
			},
			{
				week: 'Weeks 5–6',
				title: 'Community outreach',
				description: 'Serving and reaching your community.'
			},
			{
				week: 'Weeks 7–8',
				title: 'Discipleship',
				description: 'Following up and grounding new believers.'
			}
		]
	}
];

/** Get course by backend id or slug (same as id) */
export function getCourseBySlug(slug: string): CourseDetail | undefined {
	return COURSE_CATALOG.find((c) => c.id === slug);
}

/** Resolve backend course id to our catalog detail (for dashboard, etc.) */
export function getCourseDetailById(id: string): CourseDetail | undefined {
	return COURSE_CATALOG.find((c) => c.id === id);
}

/** Program duration and schedule text for display */
export function getProgramSchedule(): string {
	return `${SITE.duration}, ${SITE.schedule}`;
}
