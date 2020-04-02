import { Component, OnInit } from '@angular/core';
import { HttpClient} from '@angular/common/http';
import { ActivatedRoute } from "@angular/router";

import backendURL from '../backendURL';

@Component({
  selector: 'app-countdown',
  templateUrl: './countdown.component.html',
  styleUrls: ['./countdown.component.css']
})
export class CountdownComponent implements OnInit {

  theme: boolean = false;
  timeRemaining: Object[] = [{time: "00", unit: "seconds"}];

  id: string = "";
  name: string = "";
  timestamp: string;

  constructor(private http: HttpClient, private route: ActivatedRoute) { }

  ngOnInit() {
    this.id = this.route.snapshot.paramMap.get("id");

    this.http.post<{
      success: boolean;
      errors: [];
      id: string;
      name: string;
      datetime: string;
    }>(backendURL + "/get-countdown",
    {
      "id": {"data": this.id, "id": "id"},
    })
    .subscribe(
      data  => {
        console.log("POST Request is successful ", data);
        if (data.success) {
          this.id = data.id;
          this.name = data.name;
          console.log(data.datetime);
          this.timestamp = data.datetime;
          var timer = setInterval(() => this.updateClock(), 10);
        } else {
          // this.errService.handleErrors(this.el, data.errors, ["email", "password"]);
        }
      },
      error  => {
        console.log("Error", error);
        // this.errService.handleErrors(this.el, [{"id": "fatal", "reason": "Unable to perform request. Please try again."}], []);
      }

    );
  }

  padZeros(data, desiredLength) {
    while (data.toString().length < desiredLength) {
      data = "0" + data.toString();
    }
    return data;
  }
  
  updateClock() {
    var currentDate = new Date();
    var targetTime = new Date(this.timestamp);
    var millisecondsDelta = targetTime.getTime() - currentDate.getTime();
    var d = new Date(Date.UTC(0, 0, 0, 0, 0, 0, millisecondsDelta));

    if (millisecondsDelta <= 0)
      millisecondsDelta = 0;

    var days = this.padZeros(Math.floor(millisecondsDelta / 1000 / 60 / 60 / 24), 2);
    var hours = this.padZeros(Math.floor(millisecondsDelta / 1000 / 60 / 60) % 24, 2);
    var minutes = this.padZeros(Math.floor(millisecondsDelta / 1000 / 60) % 60, 2);
    var seconds = this.padZeros(Math.floor(millisecondsDelta / 1000) % 60, 2);
    var ms = this.padZeros(Math.floor(millisecondsDelta), 3);

    var time = [];
    var addRest = false;

    if (days > 0 || addRest) {
      time.push({time: days, unit: "days"});
      time.push({time: ":", unit: ""});
      addRest = true;
    }
    if (hours > 0 || addRest) {
      time.push({time: hours, unit: "hours"});
      time.push({time: ":", unit: ""});
      addRest = true;
    }
    if (minutes > 0 || addRest) {
      time.push({time: minutes, unit: "minutes"});
      time.push({time: ":", unit: ""});
      addRest = true;
    }
    if (seconds > 0 || addRest)
      time.push({time: seconds, unit: "seconds"});

    // If countdown is complete, display 0 seconds.
    if (millisecondsDelta <= 0)
      time.push({time: "0", unit: "seconds"});

    this.timeRemaining = time;
  }
  
  themeChange() {
    if (this.theme) {
      document.body.setAttribute('data-theme', 'light')
    } else {
      document.body.setAttribute('data-theme', 'dark')
    }
  }

}