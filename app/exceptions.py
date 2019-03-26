from flask import jsonify


class USException(Exception):

    def __init__(self, message=None, payload=None):
        Exception.__init__(self)
        if message is not None:
            self.message = message
        self.payload = payload

    def to_dict(self):
        rv = dict(self.payload or ())
        rv['error_message'] = self.message
        rv['error_name'] = self.__class__.__name__
        rv['error_code'] = self.code
        return rv

    def __str__(self):
        return "%s" % (self.message,)


class InvalidRequest(USException):
    code = 'US1'
    message = 'Request validation has failed'


class InvalidEvent(USException):
    code = 'US2'
    message = 'Event %s is invalid. Reason: "%s"'

    def __init__(self, event, reason):
        self.message = self.message % (event, reason)
        super(InvalidEvent, self).__init__()
