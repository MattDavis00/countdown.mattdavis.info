//////////////////////////////////// Dependencies ////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
"use strict";

var Validation = require("./validation");

const express = require('express');
var mysql = require('mysql');
var con = require('./connect');

var cred = require('./credentials');

const app = express();
const port = 4000;

// Parse body of request as a JSON object.
app.use(express.json());

// Do not cache any server-side authentication requests.
// Client must check again each time to ensure consistency.
const nocache = require('nocache');
app.use(nocache());

// Heath Check
app.listen(port, () => console.log(`Auth is listening on port ${port}!`));

app.get('/', function(req, res){
    res.send("Countdown backend is currently running!");
 });


 //////////////////////////////////// Code ////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

try {

    /////////////////////// Create Countdown ////////////////////////
    app.post('/create', function (req, res) {
        var request = req.body;
        console.log(req.body);

        var val = new Validation();
        val.name(request.name);
        val.date(request.datetime);
        if (val.getOutput().success) {

            var countdownID = randomID(4);

            var timestamp = new Date(request.datetime.data);
            var sql = "INSERT INTO countdown (ID, Name, DateTime) VALUES (?,?,?)";
            con.query(sql, [countdownID, request.name.data, timestamp], function (err, result) {
                if (!err)
                {
                    console.log("Number of records inserted: " + result.affectedRows);
                    if (result.affectedRows === 1) {
                        res.send(JSON.stringify({"success": true, "errors": [], "id": countdownID}));
                    } else {
                        errorHandler(err, res);
                    }
                } else {
                    errorHandler(err, res);
                }
            });

        } else {
            val.sendResponse(res);
        }
        
    });

    /////////////////////// Create Countdown ////////////////////////
    app.post('/get-countdown', function (req, res) {
        var request = req.body;
        console.log(req.body);

        var id = request.id.data;

        var sql = "SELECT id, name, datetime FROM countdown WHERE id = ?";
        con.query(sql, [request.id.data], function (err, result) {
            if (!err)
            {
                console.log("Number of records retrieved: " + result.length);
                if (result.length === 1) {
                    res.send(JSON.stringify({"success": true, "errors": [], "id": result[0].id, "name": result[0].name, "datetime": result[0].datetime}));
                } else {
                    res.send(JSON.stringify({"success": false, "errors": [{"id": "all", "reason": "Email and/or password are incorrect."}]}));
                }
            } else {
                errorHandler(err, res);
            }
        });
    
    });


    /////////////////////// Login ////////////////////////
    app.post('/', function (req, res){
        login(req, res);
    });

    function login(req, res) {
        var request = req.body;
        console.log(req.body);

        var sql = "SELECT User_ID, Email, Password_Hash, First_Name, Last_Name FROM User WHERE Email = ?";
        con.query(sql, [request.email.data], function (err, result) {
            if (!err)
            {
                console.log("Number of records retrieved: " + result.length);
                if (result.length === 1 &&  bcrypt.compareSync(request.password.data, result[0].Password_Hash)) {
                    req.session.loggedIn = true;
                    req.session.userID = result[0].User_ID;
                    req.session.email = result[0].Email;
                    req.session.firstName = result[0].First_Name;
                    req.session.lastName = result[0].Last_Name;
                    req.session.hash = result[0].Password_Hash;

                    res.send(JSON.stringify({"success": true, "errors": []}));
                } else {
                    res.send(JSON.stringify({"success": false, "errors": [{"id": "all", "reason": "Email and/or password are incorrect."}]}));
                }
            } else {
                errorHandler(err, res);
            }
        });
    }

    /////////////////////// Error Handler ////////////////////////
    function errorHandler (err, res) {
        console.log(err);
        if (res !== undefined)
            res.send(JSON.stringify({"success": false, "errors": [{"id": "fatal", "reason": "Unable to perform request. Please try again."}]}));
    }

    /**
     * Generates a random string id for a given length
     * Should be used for short strings only
     * 
     * Adapted from: https://stackoverflow.com/a/1349426
     * 
     * @param {int} length The length of the random string
     */
    function randomID(length) {
        var result = '';
        var characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
        var charactersLength = characters.length;
        for ( var i = 0; i < length; i++ ) {
            result += characters.charAt(Math.floor(Math.random() * charactersLength));
        }
        return result;
    }


}
catch(err)
{
    console.log(err);
}